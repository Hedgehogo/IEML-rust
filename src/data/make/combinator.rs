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

pub fn null<O, E>(begin_mark: Mark, output: O) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
{
    move |token| Ok((token.maker.add(begin_mark, Node::Null), output))
}

pub fn raw<O, E, S>(
    begin_mark: Mark,
    output: O,
    raw: S,
) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token| Ok((token.maker.add(begin_mark, Node::Raw(raw.into())), output))
}

pub fn string<O, E, S>(
    begin_mark: Mark,
    output: O,
    string: S,
) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token| {
        let used_token = token.maker.add(begin_mark, Node::String(string.into()));
        Ok((used_token, output))
    }
}

pub fn list<O, E, F>(begin_mark: Mark, f: F) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(ListToken) -> marked::MakeListResult<O, E>,
{
    move |token| {
        let list_token = ListToken::new(token.maker);
        return match f(list_token) {
            Ok((list_token, output)) => {
                let maker = list_token.maker;
                let result = list_token.result;
                let used_token = maker.add(begin_mark, Node::List(ListNode::new(result)));
                Ok((used_token, output))
            }
            Err((list_token, error)) => Err((Token::new(list_token.maker), error)),
        };
    }
}

pub fn map<O, E, F>(begin_mark: Mark, f: F) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(MapToken) -> marked::MakeMapResult<O, E>,
{
    move |token| {
        let map_token = MapToken::new(token.maker);
        return match f(map_token) {
            Ok((map_token, output)) => {
                let maker = map_token.maker;
                let result = map_token.result;
                let used_token = maker.add(begin_mark, Node::Map(MapNode::new(result)));
                Ok((used_token, output))
            }
            Err((map_token, error)) => Err((Token::new(map_token.maker), error)),
        };
    }
}

pub fn tagged<O, E, F, S>(
    begin_mark: Mark,
    tag: S,
    f: F,
) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::MakeResult<O, E>,
    S: Into<String>,
{
    move |token| {
        f(token).map(|(used_token, output)| {
            let maker = used_token.maker;
            let index = used_token.index;
            let result = TaggedNode::new(tag.into(), index);
            let used_token = maker.add(begin_mark, Node::Tagged(result));
            (used_token, output)
        })
    }
}

pub fn file<O, E, F, A>(
    begin_mark: Mark,
    path: PathBuf,
    anchors: A,
    f: F,
) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::MakeResult<O, E>,
    A: FnOnce(MapToken) -> marked::MakeMapResult<O, E>,
{
    move |token| {
        let map_token = MapToken::new(token.maker);
        let (maker, file_anchors, output) = match anchors(map_token) {
            Ok((map_token, output)) => (map_token.maker, map_token.result, output),
            Err((map_token, error)) => return Err((Token::new(map_token.maker), error)),
        };
        let (maker, result) = maker.child(|maker| match f(Token::new(maker)) {
            Ok((used_token, _)) => {
                let file = FileNode::new(
                    path,
                    used_token.index,
                    std::mem::take(used_token.maker.anchors()),
                    MapNode::new(file_anchors),
                    None,
                );
                (used_token.maker, Ok(file))
            }
            Err((token, error)) => (token.maker, Err(error)),
        });
        match result {
            Ok(file) => {
                let used_token = maker.add(begin_mark, Node::File(file));
                Ok((used_token, output))
            }
            Err(error) => Err((Token::new(maker), error)),
        }
    }
}

pub fn take_anchor<O, E, F, S>(
    begin_mark: Mark,
    name: S,
    f: F,
) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::MakeResult<O, E>,
    S: Into<String>,
{
    move |token| {
        f(Token::new(token.maker)).and_then(|(used_token, output)| {
            let name = name.into();
            let maker = used_token.maker;
            let index = used_token.index;
            let result = TakeAnchorNode::new(name.clone(), index);
            match maker.add_anchor(name.clone(), index) {
                Some(_) => {
                    let used_token = maker.add(begin_mark, Node::TakeAnchor(result));
                    Ok((used_token, output))
                }
                None => {
                    let reason = MakeErrorReason::AnchorAlreadyExist(name);
                    let error = marked::MakeError::new_with(begin_mark, maker.path(), reason);
                    Err((Token::new(maker), error))
                }
            }
        })
    }
}

pub fn get_anchor<O, E, S>(
    begin_mark: Mark,
    output: O,
    name: S,
) -> impl FnOnce(Token) -> marked::MakeResult<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token| {
        let result = GetAnchorNode::new(name.into(), 0);
        let used_token = token.maker.add(begin_mark, Node::GetAnchor(result));
        Ok((used_token, output))
    }
}

pub fn make<O, E, F>(begin_mark: Mark, f: F) -> Result<(Data, O), marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::MakeResult<O, E>,
{
    let mut maker = Maker::new(PathBuf::new());
    let (_, result) = maker.child(|maker| match f(Token::new(maker)) {
        Ok((used_token, output)) => {
            let file = FileNode {
                node_index: used_token.index,
                anchors: std::mem::take(used_token.maker.anchors()),
                ..Default::default()
            };
            (used_token.maker, Ok((file, output)))
        }
        Err((token, error)) => (token.maker, Err(error)),
    });
    result.and_then(|(file, output)| {
        maker.add(begin_mark, Node::File(file));
        let mut data = maker.data();
        init(&mut data)?;
        Ok((data, output))
    })
}

pub fn make_file<O, E, F, A>(
    begin_mark: Mark,
    path: PathBuf,
    anchors: A,
    f: F,
) -> Result<(Data, O), marked::MakeError<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::MakeResult<O, E>,
    A: FnOnce(MapToken) -> marked::MakeMapResult<O, E>,
{
    let mut maker = Maker::new(path.clone());
    match file(begin_mark, path, anchors, f)(Token::new(&mut maker)) {
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
            list(begin_mark, |list_token| {
                let (list_token, _) = list_token.add(raw(begin_mark, (), "hello"))?;
                let (list_token, _) = list_token.add(string(begin_mark, (), "hello"))?;
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
            map(begin_mark, |map_token| {
                let (map_token, _) =
                    map_token.add(begin_mark, "first", raw(begin_mark, (), "hello"))?;
                let (map_token, _) =
                    map_token.add(begin_mark, "second", string(begin_mark, (), "hello"))?;
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
            make::<_, Infallible, _>(begin_mark, tagged(begin_mark, "tag", null(begin_mark, ())))
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
                |map_token| {
                    let (map_token, _) =
                        map_token.add(begin_mark, "file-anchor", null(begin_mark, ()))?;
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
