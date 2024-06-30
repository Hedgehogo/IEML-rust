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
};
use std::{error::Error, path::PathBuf};

pub use super::maker::Maker;

pub fn null<O, E>(
    begin_mark: Mark,
    output: O,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
{
    move |maker| {
        maker.add(begin_mark, Node::Null);
        Ok(output)
    }
}

pub fn raw<O, E, S>(
    begin_mark: Mark,
    output: O,
    raw: S,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        maker.add(begin_mark, Node::Raw(raw.into()));
        Ok(output)
    }
}

pub fn string<O, E, S>(
    begin_mark: Mark,
    output: O,
    string: S,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        maker.add(begin_mark, Node::String(string.into()));
        Ok(output)
    }
}

pub fn list<O, E, F, I>(
    begin_mark: Mark,
    output: O,
    iter: I,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    I: Iterator<Item = F>,
{
    move |maker| {
        let mut output = output;
        let result: Result<_, _> = iter
            .map(|f| {
                f(maker).map(|mark| {
                    output = mark;
                    maker.last()
                })
            })
            .collect();
        result.map(|i| {
            maker.add(begin_mark, Node::List(ListNode::new(i)));
            output
        })
    }
}

pub fn map<O, E, F, S, I>(
    begin_mark: Mark,
    output: O,
    iter: I,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
    I: Iterator<Item = (S, F)>,
{
    move |maker| {
        let mut output = output;
        let result: Result<_, _> = iter
            .map(|(key, f)| {
                f(maker).map(|mark| {
                    output = mark;
                    (key.into(), maker.last())
                })
            })
            .collect();
        result.map(|i| {
            maker.add(begin_mark, Node::Map(MapNode::new(i)));
            output
        })
    }
}

pub fn tag<O, E, F, S>(
    begin_mark: Mark,
    tag: S,
    f: F,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
{
    move |maker| {
        f(maker).map(|output| {
            let result = TaggedNode::new(tag.into(), maker.last());
            maker.add(begin_mark, Node::Tagged(result));
            output
        })
    }
}

pub fn file<O, E, F, A, S, I>(
    begin_mark: Mark,
    output: O,
    path: PathBuf,
    anchors: I,
    f: F,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    A: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    move |maker| {
        let mut output = output;
        let file_anchors = anchors
            .map(|(key, f)| {
                f(maker).map(|mark| {
                    output = mark;
                    (key.into(), maker.last())
                })
            })
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
        maker.add(begin_mark, Node::File(result));
        Ok(output)
    }
}

pub fn take_anchor<O, E, F, S>(
    begin_mark: Mark,
    name: S,
    f: F,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
{
    move |maker| {
        f(maker).and_then(|output| {
            let name = name.into();
            let result = TakeAnchorNode::new(name.clone(), maker.last());
            maker
                .add_anchor(name.clone(), maker.last())
                .ok_or(marked::MakeError::new(
                    begin_mark,
                    MakeError::new(
                        maker.path().to_path_buf(),
                        MakeErrorReason::AnchorAlreadyExist(name),
                    ),
                ))?;
            maker.add(begin_mark, Node::TakeAnchor(result));
            Ok(output)
        })
    }
}

pub fn get_anchor<O, E, S>(
    begin_mark: Mark,
    output: O,
    name: S,
) -> impl FnOnce(&mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |maker| {
        let result = GetAnchorNode::new(name.into(), 0);
        maker.add(begin_mark, Node::GetAnchor(result));
        Ok(output)
    }
}

pub fn make<O, E, F>(begin_mark: Mark, f: F) -> Result<Data, marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
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
    maker.add(begin_mark, Node::File(result));
    init(&mut data)?;
    Ok(data)
}

pub fn make_file<O, E, F, A, S, I>(
    begin_mark: Mark,
    output: O,
    path: PathBuf,
    anchors: I,
    f: F,
) -> Result<Data, marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    A: FnOnce(&mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    let mut data = Data::default();
    let mut maker = Maker::new(&mut data, path.clone());
    file(begin_mark, output, path, anchors, f)(&mut maker)?;
    init(&mut data)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::super::super::node_type::NodeType;
    use super::*;
    use std::convert::Infallible;

    #[test]
    fn test_null() {
        let begin_mark = Mark::default();
        let data =
            make::<_, Infallible, _>(begin_mark, null(begin_mark, ()))
                .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Null);
    }

    #[test]
    fn test_raw() {
        let begin_mark = Mark::default();
        let data = make::<_, Infallible, _>(
            begin_mark,
            raw(begin_mark, (), "hello"),
        )
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Raw);
        assert_eq!(clear_view.raw().unwrap().raw(), "hello");
    }

    #[test]
    fn test_string() {
        let begin_mark = Mark::default();
        let data = make::<_, Infallible, _>(
            begin_mark,
            string(begin_mark, (), "hello"),
        )
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::String);
        assert_eq!(clear_view.string().unwrap().string(), "hello");
    }

    #[test]
    fn test_list() {
        let begin_mark = Mark::default();
        let data = make::<_, Infallible, _>(begin_mark, {
            list(
                begin_mark,
                (),
                [
                    Box::new(raw(begin_mark, (), "hello"))
                        as Box<dyn FnOnce(&mut Maker) -> marked::MakeResult<_, Infallible>>,
                    Box::new(string(begin_mark, (), "hello")),
                ]
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
        let begin_mark = Mark::default();
        let data = make::<_, Infallible, _>(begin_mark, {
            map(
                begin_mark,
                (),
                [
                    (
                        "first",
                        Box::new(raw(begin_mark, (), "hello"))
                            as Box<dyn FnOnce(&mut Maker) -> marked::MakeResult<_, Infallible>>,
                    ),
                    (
                        "second",
                        Box::new(string(begin_mark, (), "hello")),
                    ),
                ]
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
        let begin_mark = Mark::default();
        let data = make::<_, Infallible, _>(begin_mark, {
            tag(
                begin_mark,
                "tag",
                null(begin_mark, ()),
            )
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
        let begin_mark = Mark::default();
        let data = make::<_, Infallible, _>(begin_mark, {
            file(
                begin_mark,
                (),
                "dir/name.ieml".into(),
                [(
                    "file-anchor",
                    Box::new(null(begin_mark, ()))
                        as Box<dyn FnOnce(&mut Maker) -> marked::MakeResult<_, Infallible>>,
                )]
                .into_iter(),
                raw(begin_mark, (), "hello"),
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
