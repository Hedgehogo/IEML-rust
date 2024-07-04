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

pub use super::maker::{ListToken, Maker, MapToken, Token};

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

pub fn list<O, E, F>(
    begin_mark: Mark,
    f: F,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: for<'maker> FnOnce(ListToken<'maker>) -> marked::ListMakeResult<'maker, O, E>,
{
    move |token, maker| {
        let list_token = ListToken::new(maker);
        return match f(list_token) {
            Ok((list_token, output)) => {
                let maker = list_token.maker;
                let result = list_token.result;
                let added = maker.add(begin_mark, token, Node::List(ListNode::new(result)));
                Ok((added, output))
            }
            Err(error) => Err((token, error)),
        };
    }
}

pub fn map<O, E, F>(
    begin_mark: Mark,
    f: F,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: for<'maker> FnOnce(MapToken<'maker>) -> marked::MapMakeResult<'maker, O, E>,
{
    move |token, maker| {
        let map_token = MapToken::new(maker);
        return match f(map_token) {
            Ok((map_token, output)) => {
                let maker = map_token.maker;
                let result = map_token.result;
                let added = maker.add(begin_mark, token, Node::Map(MapNode::new(result)));
                Ok((added, output))
            }
            Err(error) => Err((token, error)),
        };
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

pub fn file<O, E, F, A>(
    begin_mark: Mark,
    path: PathBuf,
    anchors: A,
    f: F,
) -> impl FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    A: for<'maker> FnOnce(MapToken<'maker>) -> marked::MapMakeResult<'maker, O, E>,
{
    move |token, maker| {
        let map_token = MapToken::new(maker);
        let (maker, file_anchors, output) = match anchors(map_token) {
            Ok((map_token, output)) => (map_token.maker, map_token.result, output),
            Err(error) => return Err((token, error)),
        };
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

pub fn make_file<O, E, F, A>(
    begin_mark: Mark,
    path: PathBuf,
    anchors: A,
    f: F,
) -> Result<(Data, O), marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    A: for<'maker> FnOnce(MapToken<'maker>) -> marked::MapMakeResult<'maker, O, E>,
{
    let mut maker = Maker::new(path.clone());
    match file(begin_mark, path, anchors, f)(Token::new(), &mut maker) {
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
        let begin_mark = Mark::default();
        let (data, _) = make::<_, Infallible, _>(
            begin_mark,
            list(begin_mark, |mut list_token| {
                list_token.add(raw(begin_mark, (), "hello"))?;
                list_token.add(string(begin_mark, (), "hello"))?;
                Ok((list_token, ()))
            }),
        )
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
        let (data, _) = make::<_, Infallible, _>(
            begin_mark,
            map(begin_mark, |mut map_token| {
                map_token.add(begin_mark, "first".into(), raw(begin_mark, (), "hello"))?;
                map_token.add(begin_mark, "second".into(), string(begin_mark, (), "hello"))?;
                Ok((map_token, ()))
            }),
        )
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
                "dir/name.ieml".into(),
                |mut map_token| {
                    map_token.add(begin_mark, "file-anchor".into(), null(begin_mark, ()))?;
                    Ok((map_token, ()))
                },
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
