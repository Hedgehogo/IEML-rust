use super::{
    super::{
        data::Data,
        mark::Mark,
        node::node::{
            FileNode, GetAnchorNode, ListNode, MapNode, Node, TaggedNode, TakeAnchorNode,
        },
    },
    error::*,
    init::init,
    maker::Maker,
};
use std::{error::Error, path::PathBuf};

pub type MakeResult<E> = Result<(), marked::MakeError<E>>;

pub fn null<E>(mark: Mark) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
{
    move |maker| {
        maker.add(mark, Node::Null);
        Ok(())
    }
}

pub fn raw<E, S>(mark: Mark, raw: S) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        maker.add(mark, Node::Raw(raw.into()));
        Ok(())
    }
}

pub fn string<E, S>(mark: Mark, string: S) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| Ok(maker.add(mark, Node::String(string.into())))
}

pub fn list<E, F, I>(mark: Mark, iter: I) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    I: Iterator<Item = F>,
{
    move |maker| {
        let result: Result<_, _> = iter.map(|f| f(maker).map(|_| maker.last())).collect();
        result.map(|i| maker.add(mark, Node::List(ListNode::new(i))))
    }
}

pub fn map<E, F, S, I>(mark: Mark, iter: I) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
    I: Iterator<Item = (S, F)>,
{
    move |maker| {
        let result: Result<_, _> = iter
            .map(|(key, f)| f(maker).map(|_| (key.into(), maker.last())))
            .collect();
        result.map(|i| maker.add(mark, Node::Map(MapNode::new(i))))
    }
}

pub fn tag<E, F, S>(mark: Mark, tag: S, f: F) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
{
    move |maker| {
        f(maker)?;
        let result = TaggedNode::new(tag.into(), maker.last());
        maker.add(mark, Node::Tagged(result));
        Ok(())
    }
}

pub fn file<E, F, A, S, I>(
    mark: Mark,
    path: PathBuf,
    anchors: I,
    f: F,
) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    A: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    move |maker| {
        let file_anchors = anchors
            .map(|(key, f)| f(maker).map(|_| (key.into(), maker.last())))
            .collect::<Result<_, _>>()?;
        let result = maker.child(|maker| {
            f(maker).map(|_| {
                FileNode::new(
                    path,
                    maker.last(),
                    std::mem::take(maker.anchors()),
                    MapNode::new(file_anchors),
                    None,
                )
            })
        })?;
        maker.add(mark, Node::File(result));
        Ok(())
    }
}

pub fn take_anchor<E, F, S>(mark: Mark, name: S, f: F) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
{
    move |maker| {
        f(maker)?;
        let name = name.into();
        let result = TakeAnchorNode::new(name.clone(), maker.last());
        maker
            .add_anchor(name.clone(), maker.last())
            .ok_or(marked::MakeError::new(
                mark,
                MakeError::new(
                    maker.path().to_path_buf(),
                    MakeErrorReason::AnchorAlreadyExist(name),
                ),
            ))?;
        maker.add(mark, Node::TakeAnchor(result));
        Ok(())
    }
}

pub fn get_anchor<E, S>(mark: Mark, name: S) -> impl FnOnce(&mut Maker) -> MakeResult<E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        let result = GetAnchorNode::new(name.into(), 0);
        maker.add(mark, Node::GetAnchor(result));
        Ok(())
    }
}

pub fn make<E, F>(mark: Mark, f: F) -> Result<Data, marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
{
    let mut data = Data::default();
    let mut maker = Maker::new(&mut data, PathBuf::new());
    let result = maker.child(|maker| {
        f(maker).map(|_| FileNode {
            node_index: maker.last(),
            anchors: std::mem::take(maker.anchors()),
            ..Default::default()
        })
    })?;
    maker.add(mark, Node::File(result));
    init(&mut data)?;
    Ok(data)
}

pub fn make_file<E, F, A, S, I>(
    mark: Mark,
    path: PathBuf,
    anchors: I,
    f: F,
) -> Result<Data, marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> MakeResult<E>,
    A: FnOnce(&mut Maker) -> MakeResult<E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    let mut data = Data::default();
    let mut maker = Maker::new(&mut data, path.clone());
    file(mark, path, anchors, f)(&mut maker)?;
    init(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::super::super::node_type::NodeType;
    use super::*;
    use std::collections::HashMap;
    use std::convert::Infallible;

    #[test]
    fn test_null() {
        let data = make::<Infallible, _>(Mark::default(), null(Mark::default())).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Null);
    }

    #[test]
    fn test_raw() {
        let data = make::<Infallible, _>(Mark::default(), raw(Mark::default(), "hello")).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Raw);
        assert_eq!(clear_view.raw().unwrap().raw(), "hello");
    }

    #[test]
    fn test_string() {
        let data =
            make::<Infallible, _>(Mark::default(), string(Mark::default(), "hello")).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::String);
        assert_eq!(clear_view.string().unwrap().string(), "hello");
    }

    #[test]
    fn test_list() {
        let data = make::<Infallible, _>(Mark::default(), {
            list(
                Mark::default(),
                Vec::from([
                    Box::new(raw(Mark::default(), "hello"))
                        as Box<dyn FnOnce(&mut Maker) -> MakeResult<Infallible>>,
                    Box::new(string(Mark::default(), "hello")),
                ])
                .into_iter(),
            )
        })
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::List);

        let list = clear_view.list().unwrap();
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(0).unwrap().node_type(), NodeType::Raw);
        assert_eq!(list.get(1).unwrap().node_type(), NodeType::String);
    }

    #[test]
    fn test_map() {
        let data = make::<Infallible, _>(Mark::default(), {
            map(
                Mark::default(),
                HashMap::from([
                    (
                        "first",
                        Box::new(raw(Mark::default(), "hello"))
                            as Box<dyn FnOnce(&mut Maker) -> MakeResult<Infallible>>,
                    ),
                    ("second", Box::new(string(Mark::default(), "hello"))),
                ])
                .into_iter(),
            )
        })
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Map);

        let map = clear_view.map().unwrap();
        assert_eq!(map.len(), 2);
        assert_eq!(map.get("first").unwrap().node_type(), NodeType::Raw);
        assert_eq!(map.get("second").unwrap().node_type(), NodeType::String);
    }

    #[test]
    fn test_tagged() {
        let data = make::<Infallible, _>(Mark::default(), {
            tag(Mark::default(), "tag", null(Mark::default()))
        })
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Tagged);
        assert_eq!(view.tagged().unwrap().tag(), "tag");

        assert!(view.is_null());
    }

    #[test]
    fn test_file() {
        let data = make::<Infallible, _>(Mark::default(), {
            file(
                Mark::default(),
                "dir/name.ieml".into(),
                HashMap::from([(
                    "file-anchor",
                    Box::new(null(Mark::default()))
                        as Box<dyn FnOnce(&mut Maker) -> MakeResult<Infallible>>,
                )])
                .into_iter(),
                raw(Mark::default(), "hello"),
            )
        })
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::File);
        assert_eq!(
            clear_view.file().unwrap().path(),
            PathBuf::from("dir/name.ieml").as_path()
        );

        let anchors = clear_view.file().unwrap().anchors().file_anchors();
        assert_eq!(anchors.len(), 1);
        assert!(anchors.contains_key(&"file-anchor".into()));

        assert!(clear_view.is_raw());
        assert_eq!(clear_view.raw().unwrap().raw(), "hello");
    }
}
