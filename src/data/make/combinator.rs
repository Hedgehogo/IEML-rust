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

pub use super::maker::{Maker, Token};

pub fn null<O, E>(
    begin_mark: Mark,
    output: O,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
{
    move |token, maker| Ok((maker.add(begin_mark, token, Node::Null), output))
}

pub fn raw<O, E, S>(
    begin_mark: Mark,
    output: O,
    raw: S,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token, maker| Ok((maker.add(begin_mark, token, Node::Raw(raw.into())), output))
}

pub fn string<O, E, S>(
    begin_mark: Mark,
    output: O,
    string: S,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token, maker| {
        let added = maker.add(begin_mark, token, Node::String(string.into()));
        Ok((added, output))
    }
}

pub fn list<O, E, F, I>(
    begin_mark: Mark,
    output: O,
    iter: I,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    I: Iterator<Item = F>,
{
    move |token, maker| {
        let mut output = output;
        let result: Result<_, _> = iter
            .map(|f| {
                f(Token::new(), maker).map(|(added, end_output)| {
                    output = end_output;
                    added.index()
                })
            })
            .collect();
        result.map(|i| {
            let added = maker.add(begin_mark, token, Node::List(ListNode::new(i)));
            (added, output)
        })
    }
}

pub fn map<O, E, F, S, I>(
    begin_mark: Mark,
    output: O,
    iter: I,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
    I: Iterator<Item = (S, F)>,
{
    move |token, maker| {
        let mut output = output;
        let result: Result<_, _> = iter
            .map(|(key, f)| {
                f(Token::new(), maker).map(|(added, end_output)| {
                    output = end_output;
                    (key.into(), added.index())
                })
            })
            .collect();
        result.map(|i| {
            let added = maker.add(begin_mark, token, Node::Map(MapNode::new(i)));
            (added, output)
        })
    }
}

pub fn tag<O, E, F, S>(
    begin_mark: Mark,
    tag: S,
    f: F,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
{
    move |token, maker| {
        f(Token::new(), maker).map(|(added, output)| {
            let result = TaggedNode::new(tag.into(), added.index());
            let added = maker.add(begin_mark, token, Node::Tagged(result));
            (added, output)
        })
    }
}

pub fn file<O, E, F, A, S, I>(
    begin_mark: Mark,
    output: O,
    path: PathBuf,
    anchors: I,
    f: F,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    A: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    move |token, maker| {
        let mut output = output;
        let file_anchors = anchors
            .map(|(key, f)| {
                f(Token::new(), maker).map(|(added, end_output)| {
                    output = end_output;
                    (key.into(), added.index())
                })
            })
            .collect::<Result<_, _>>()?;
        let result = maker.child(|maker| {
            f(Token::new(), maker).map(|(added, _)| {
                FileNode::new(
                    path,
                    added.index(),
                    std::mem::take(maker.anchors()),
                    MapNode::new(file_anchors),
                    None,
                )
            })
        })?;
        let added = maker.add(begin_mark, token, Node::File(result));
        Ok((added, output))
    }
}

pub fn take_anchor<O, E, F, S>(
    begin_mark: Mark,
    name: S,
    f: F,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
{
    move |token, maker| {
        f(Token::new(), maker).and_then(|(added, output)| {
            let name = name.into();
            let index = added.index();
            let result = TakeAnchorNode::new(name.clone(), index);
            match maker.add_anchor(name.clone(), index) {
                Some(_) => {
                    let added = maker.add(begin_mark, token, Node::TakeAnchor(result));
                    Ok((added, output))
                }
                None => {
                    let reason = MakeErrorReason::AnchorAlreadyExist(name);
                    let error = marked::MakeError::new_with(begin_mark, maker.path(), reason);
                    Err((token, error))
                }
            }
        })
    }
}

pub fn get_anchor<O, E, S>(
    begin_mark: Mark,
    output: O,
    name: S,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token, maker| {
        let result = GetAnchorNode::new(name.into(), 0);
        let added = maker.add(begin_mark, token, Node::GetAnchor(result));
        Ok((added, output))
    }
}

pub fn make<O, E, F>(begin_mark: Mark, f: F) -> Result<(Data, O), marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
{
    let mut maker = Maker::new(PathBuf::new());
    let (result, output) = maker
        .child(|maker| {
            f(Token::new(), maker).map(|(added, output)| {
                let file = FileNode {
                    node_index: added.index(),
                    anchors: std::mem::take(maker.anchors()),
                    ..Default::default()
                };
                (file, output)
            })
        })
        .map_err(|(_, error)| error)?;
    maker.add(begin_mark, Token::new(), Node::File(result));

    let mut data = maker.data();
    init(&mut data)?;
    Ok((data, output))
}

pub fn make_file<O, E, F, A, S, I>(
    begin_mark: Mark,
    output: O,
    path: PathBuf,
    anchors: I,
    f: F,
) -> Result<(Data, O), marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    A: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    S: Into<String>,
    I: Iterator<Item = (S, A)>,
{
    let mut maker = Maker::new(path.clone());
    match file(begin_mark, output, path, anchors, f)(Token::new(), &mut maker) {
        Ok((_added, output)) => {
            let mut data = maker.data();
            init(&mut data)?;
            Ok((data, output))
        }
        Err((_, error)) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::node_type::NodeType;
    use super::*;
    use std::convert::Infallible;

    #[test]
    fn test_null() {
        let begin_mark = Mark::default();
        let (data, _) = make::<_, Infallible, _>(begin_mark, null(begin_mark, ())).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Null);
    }

    #[test]
    fn test_raw() {
        let begin_mark = Mark::default();
        let (data, _) = make::<_, Infallible, _>(begin_mark, raw(begin_mark, (), "hello")).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Raw);
        assert_eq!(clear_view.raw().unwrap().raw(), "hello");
    }

    #[test]
    fn test_string() {
        let begin_mark = Mark::default();
        let (data, _) =
            make::<_, Infallible, _>(begin_mark, string(begin_mark, (), "hello")).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::String);
        assert_eq!(clear_view.string().unwrap().string(), "hello");
    }

    #[test]
    fn test_list() {
        let raw: for<'maker> fn(_, _, _, &'maker mut Maker) -> _ =
            |mark, content, token, maker| raw(mark, (), content)(token, maker);
        let string: for<'maker> fn(_, _, _, &'maker mut Maker) -> _ =
            |mark, content, token, maker| string(mark, (), content)(token, maker);

        let begin_mark = Mark::default();
        let iter = [("hello", raw), ("hello", string)]
            .into_iter()
            .map(|(content, f)| move |token, maker| f(begin_mark, content, token, maker));
        let (data, _) = make::<_, Infallible, _>(begin_mark, list(begin_mark, (), iter)).unwrap();
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
        let raw: fn(_, _, _, _) -> _ =
            |mark, content, token, maker| raw(mark, (), content)(token, maker);
        let string: fn(_, _, _, _) -> _ =
            |mark, content, token, maker| string(mark, (), content)(token, maker);

        let begin_mark = Mark::default();
        let iter = [("first", "hello", raw), ("second", "hello", string)]
            .into_iter()
            .map(|(key, content, f)| {
                (key, move |token, maker| {
                    f(begin_mark, content, token, maker)
                })
            });
        let (data, _) = make::<_, Infallible, _>(begin_mark, map(begin_mark, (), iter)).unwrap();
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
        let (data, _) =
            make::<_, Infallible, _>(begin_mark, tag(begin_mark, "tag", null(begin_mark, ())))
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
        let (data, _) = make::<_, Infallible, _>(begin_mark, {
            file(
                begin_mark,
                (),
                "dir/name.ieml".into(),
                [("file-anchor", null(begin_mark, ()))].into_iter(),
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
