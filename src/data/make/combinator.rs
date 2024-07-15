use super::{
    super::{
        data::Data,
        mark::Mark,
        name::Name,
        node::node::{
            FileNode, GetAnchorNode, ListNode, MapNode, Node, TaggedNode, TakeAnchorNode,
        },
    },
    error::*,
    init::init,
    maker::Maker,
};
use std::{error::Error, path::PathBuf};

pub use super::maker::{ListToken, MapToken, Token};

pub fn null<O, E>(begin_mark: Mark, output: O) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
{
    move |token| Ok((token.add_node(begin_mark, Node::Null), output))
}

pub fn raw<O, E, S>(
    begin_mark: Mark,
    output: O,
    raw: S,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token| {
        let used_token = token.add_node(begin_mark, Node::Raw(raw.into()));
        Ok((used_token, output))
    }
}

pub fn string<O, E, S>(
    begin_mark: Mark,
    output: O,
    string: S,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<String>,
{
    move |token| {
        let used_token = token.add_node(begin_mark, Node::String(string.into()));
        Ok((used_token, output))
    }
}

pub fn list<O, E, F>(begin_mark: Mark, f: F) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(ListToken) -> marked::ListResult<O, E>,
{
    move |token| {
        let list_token = token.add_list();

        return match f(list_token) {
            Ok((list_token, output)) => {
                let (token, result) = list_token.split();
                let node = Node::List(ListNode::new(result));
                let used_token = token.add_node(begin_mark, node);
                Ok((used_token, output))
            }

            Err(error) => match error {
                RateError::Recoverable((list_token, error)) => {
                    let (token, result) = list_token.split();
                    if result.is_empty() {
                        Err(RateError::Recoverable((token, error)))
                    } else {
                        Err(RateError::Unrecoverable((token.error(), error)))
                    }
                }

                RateError::Unrecoverable(error) => Err(RateError::Unrecoverable(error)),
            },
        };
    }
}

pub fn map<O, E, F>(begin_mark: Mark, f: F) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(MapToken) -> marked::MapResult<O, E>,
{
    move |token| {
        let map_token = token.add_map();

        return match f(map_token) {
            Ok((map_token, output)) => {
                let (token, result) = map_token.split();
                let node = Node::Map(MapNode::new(result));
                let used_token = token.add_node(begin_mark, node);
                Ok((used_token, output))
            }

            Err(error) => match error {
                RateError::Recoverable((map_token, error)) => {
                    let (token, result) = map_token.split();
                    if result.is_empty() {
                        Err(RateError::Recoverable((token, error)))
                    } else {
                        Err(RateError::Unrecoverable((token.error(), error)))
                    }
                }

                RateError::Unrecoverable(error) => Err(RateError::Unrecoverable(error)),
            },
        };
    }
}

pub fn tagged<O, E, F, S>(
    begin_mark: Mark,
    tag: S,
    f: F,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::Result<O, E>,
    S: Into<Name>,
{
    move |token| {
        f(token).map(|(used_token, output)| {
            let (token, index) = used_token.split();
            let result = TaggedNode::new(tag.into(), index);
            let used_token = token.add_node(begin_mark, Node::Tagged(result));
            (used_token, output)
        })
    }
}

pub fn file<O, E, F, A>(
    begin_mark: Mark,
    path: PathBuf,
    anchors: A,
    f: F,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::Result<O, E>,
    A: FnOnce(MapToken) -> marked::MapResult<O, E>,
{
    move |token| {
        let map_token = token.add_map();

        let ((token, file_anchors), output) = match anchors(map_token) {
            Ok((map_token, output)) => (map_token.split(), output),

            Err(error) => match error {
                RateError::Recoverable((map_token, error)) => {
                    let (token, result) = map_token.split();
                    return if result.is_empty() {
                        Err(RateError::Recoverable((token, error)))
                    } else {
                        Err(RateError::Unrecoverable((token.error(), error)))
                    };
                }

                RateError::Unrecoverable(error) => {
                    return Err(RateError::Unrecoverable(error));
                }
            },
        };

        let result = token.child(|token| match f(token) {
            Ok((used_token, _)) => {
                let (token, index) = used_token.split();
                let (token, anchors) = token.anchors();
                let file_anchors = MapNode::new(file_anchors);
                let file = FileNode::new(path, index, anchors, file_anchors, None);
                Ok((token, file))
            }
            Err(error) => match error {
                RateError::Recoverable((token, error)) => {
                    if file_anchors.is_empty() {
                        Err(RateError::Recoverable((token, error)))
                    } else {
                        Err(RateError::Unrecoverable((token.error(), error)))
                    }
                }

                RateError::Unrecoverable(error) => Err(RateError::Unrecoverable(error)),
            },
        });

        result.map(|(token, file)| {
            let used_token = token.add_node(begin_mark, Node::File(file));
            (used_token, output)
        })
    }
}

pub fn take_anchor<O, E, F, S>(
    begin_mark: Mark,
    name: S,
    f: F,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::Result<O, E>,
    S: Into<Name>,
{
    move |token| {
        f(token).and_then(|(used_token, output)| {
            let name = name.into();
            let (token, index) = used_token.split();
            let result = TakeAnchorNode::new(name.clone(), index);

            match token.add_anchor(begin_mark, name, index) {
                Ok((token, _)) => {
                    let used_token = token.add_node(begin_mark, Node::TakeAnchor(result));
                    Ok((used_token, output))
                }

                Err(error) => Err(RateError::Recoverable(error)),
            }
        })
    }
}

pub fn get_anchor<O, E, S>(
    begin_mark: Mark,
    output: O,
    name: S,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<Name>,
{
    move |token| {
        let result = GetAnchorNode::new(name.into(), 0);
        let used_token = token.add_node(begin_mark, Node::GetAnchor(result));
        Ok((used_token, output))
    }
}

pub fn make<O, E, F>(begin_mark: Mark, f: F) -> Result<(Data, O), marked::Error<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::Result<O, E>,
{
    let mut maker = Maker::new(PathBuf::new());

    let result = Token::new(&mut maker).child(|token| {
        f(token).map(|(used_token, output)| {
            let (token, index) = used_token.split();
            let (token, anchors) = token.anchors();
            let file = FileNode {
                node_index: index,
                anchors,
                ..Default::default()
            };
            (token, (file, output))
        })
    });

    match result {
        Ok((token, (file, output))) => {
            let _ = token.add_node(begin_mark, Node::File(file));
            let mut data = maker.data();
            init(&mut data)?;
            Ok((data, output))
        }

        Err(error) => match error {
            RateError::Recoverable((_, error)) => Err(error),
            RateError::Unrecoverable((_, error)) => Err(error),
        },
    }
}

pub fn make_file<O, E, F, A>(
    begin_mark: Mark,
    path: PathBuf,
    anchors: A,
    f: F,
) -> Result<(Data, O), marked::Error<E>>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::Result<O, E>,
    A: FnOnce(MapToken) -> marked::MapResult<O, E>,
{
    let mut maker = Maker::new(path.clone());

    match file(begin_mark, path, anchors, f)(Token::new(&mut maker)) {
        Ok((_, output)) => {
            let mut data = maker.data();
            init(&mut data)?;
            Ok((data, output))
        }

        Err(error) => match error {
            RateError::Recoverable((_, error)) => Err(error),
            RateError::Unrecoverable((_, error)) => Err(error),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{name::NameRef, node_type::NodeType};
    use super::*;
    use std::convert::Infallible;

    fn name(i: &str) -> NameRef {
        NameRef::new(i).unwrap()
    }

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
                    map_token.add(begin_mark, name("first"), raw(begin_mark, (), "hello"))?;
                let (map_token, _) =
                    map_token.add(begin_mark, name("second"), string(begin_mark, (), "hello"))?;
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
        let (data, _) = make::<_, Infallible, _>(
            begin_mark,
            tagged(begin_mark, name("tag"), null(begin_mark, ())),
        )
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_file().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Tagged);
        assert_eq!(view.tagged().unwrap().tag(), name("tag"));

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
                        map_token.add(begin_mark, name("file-anchor"), null(begin_mark, ()))?;
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
