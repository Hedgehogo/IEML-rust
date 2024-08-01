use super::{
    super::{
        data::Data,
        mark::Mark,
        name::Name,
        node::node::{
            DocumentNode, AnchorRequestNode, ListNode, MapNode, Node, TaggedNode, AnchorCreationNode,
        },
    },
    error::*,
    init::init,
    maker::Maker,
};
use std::{error::Error, path::PathBuf};

pub use super::maker::{ListToken, MapToken, Token};

/// Adds a Null node.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `output` Returned by the function unchanged, added for signature consistency, which
///            makes it easy to substitute this function for other ones.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, make::null(mark, ())).unwrap();
/// assert!(data.view().is_null())
/// ```
pub fn null<O, E>(begin_mark: Mark, output: O) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
{
    move |token| Ok((token.add_node(begin_mark, Node::Null), output))
}

/// Adds a Raw node.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `output` Returned by the function unchanged, added for signature consistency, which
///            makes it easy to substitute this function for other ones.
/// * `raw` Contents.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, make::raw(mark, (), "abc")).unwrap();
/// assert_eq!(data.view().raw().unwrap().raw(), "abc")
/// ```
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

/// Adds a String node.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `output` Returned by the function unchanged, added for signature consistency, which
///            makes it easy to substitute this function for other ones.
/// * `string` Contents.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, make::string(mark, (), "abc")).unwrap();
/// assert_eq!(data.view().string().unwrap().string(), "abc")
/// ```
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

/// Adds a List node and all its child ones.
///
/// Returns the same output returned by `f`.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `f` Closure that adds child nodes.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, make::list(mark, |token| {
///     token.add(make::null(mark, ()))
/// })).unwrap();
///
/// let list_view = data.view().list().unwrap();
/// assert_eq!(list_view.len(), 1);
/// assert!(list_view.get(0).unwrap().is_null());
/// ```
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

/// Adds a Map node and all its child ones.
///
/// Returns the same output returned by `f`. Returns an unrecoverable error if `f` added at least one node and then returned an error
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `f` Closure that adds child nodes.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use serde_ieml::data::name::Name;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, make::map(mark, |token| {
///     token.add(mark, Name::new("key").unwrap(), make::null(mark, ()))
/// })).unwrap();
///
/// let map_view = data.view().map().unwrap();
/// assert_eq!(map_view.len(), 1);
/// assert!(map_view.get("key").unwrap().is_null());
/// ```
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

/// Adds a Tagged node and exactly one child node.
///
/// Returns the same output returned by `f`.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `tag` The tag that will be assigned to the child node.
/// * `f` Closure that adds a child node.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use serde_ieml::data::name::Name;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, |token| {
///     make::tagged(mark, Name::new("tag").unwrap(), make::null(mark, ()))(token)
/// }).unwrap();
///
/// let tagged_view = data.view().tagged().unwrap();
/// assert_eq!(tagged_view.tag().as_str(), "tag");
/// assert!(tagged_view.view().is_null());
/// ```
pub fn tagged<O, E, F, S>(
    begin_mark: Mark,
    tag: S,
    f: F,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::Result<O, E>,
    S: Into<Name<Box<str>>>,
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

/// Adds a Document node, its one child node, and any number of named nodes (anchors).
///
/// Returns the same output returned by `anchors`. It is assumed that the child node is in a separate document and does not affect the output.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `path` Document path or document path in the document system analogy.
/// * `anchors` Closure that adds named nodes (anchors).
/// * `f` Closure that adds a child node.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use serde_ieml::data::name::Name;
/// use std::{convert::Infallible, path::Path};
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, make::document(
///     mark, 
///     Path::new("test.ieml").to_path_buf(),
///     |token| token.add(mark, Name::new("anchor").unwrap(), make::null(mark, ())),
///     make::null(mark, ()),
/// )).unwrap();
///
/// let document_view = data.view().clear_step_document().unwrap().document().unwrap();
/// assert_eq!(document_view.path(), Path::new("test.ieml"));
/// 
/// let anchors = document_view.anchors().document_anchors();
/// assert_eq!(anchors.len(), 1);
/// assert!(anchors.get("anchor").unwrap().is_null());
/// ```
pub fn document<O, E, F, A>(
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

        let ((token, document_anchors), output) = match anchors(map_token) {
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
                let document_anchors = MapNode::new(document_anchors);
                let document = DocumentNode::new(path, index, anchors, document_anchors, None);
                Ok((token, document))
            }
            Err(error) => match error {
                RateError::Recoverable((token, error)) => {
                    if document_anchors.is_empty() {
                        Err(RateError::Recoverable((token, error)))
                    } else {
                        Err(RateError::Unrecoverable((token.error(), error)))
                    }
                }

                RateError::Unrecoverable(error) => Err(RateError::Unrecoverable(error)),
            },
        });

        result.map(|(token, document)| {
            let used_token = token.add_node(begin_mark, Node::Document(document));
            (used_token, output)
        })
    }
}

/// Adds a AnchorCreation node and its single child node.
///
/// Returns the same output returned by `f`.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `name` Name of the anchor to be created.
/// * `f` Closure that adds a child node.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use serde_ieml::data::name::Name;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, |token| {
///     make::anchor_creation(mark, Name::new("anchor").unwrap(), make::null(mark, ()))(token)
/// }).unwrap();
///
/// let anchor_creation_view = data.view().anchor_creation().unwrap();
/// assert_eq!(anchor_creation_view.name().as_str(), "anchor");
/// assert!(anchor_creation_view.view().is_null());
/// ```
pub fn anchor_creation<O, E, F, S>(
    begin_mark: Mark,
    name: S,
    f: F,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    F: FnOnce(Token) -> marked::Result<O, E>,
    S: Into<Name<Box<str>>>,
{
    move |token| {
        f(token).and_then(|(used_token, output)| {
            let name = name.into();
            let (token, index) = used_token.split();
            let result = AnchorCreationNode::new(name.clone(), index);

            match token.add_anchor(begin_mark, name, index) {
                Ok((token, _)) => {
                    let used_token = token.add_node(begin_mark, Node::AnchorCreation(result));
                    Ok((used_token, output))
                }

                Err(error) => Err(RateError::Recoverable(error)),
            }
        })
    }
}

/// Adds a AnchorRequest node.
///
/// Returns the same output returned by `f`.
///
/// *Note*: The tokens are described in more detail in [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `output` Returned by the function unchanged, added for signature consistency, which
///            makes it easy to substitute this function for other ones.
/// * `name` Name of the anchor that is being requested.
pub fn anchor_request<O, E, S>(
    begin_mark: Mark,
    output: O,
    name: S,
) -> impl FnOnce(Token) -> marked::Result<O, E>
where
    E: Error + PartialEq + Eq,
    S: Into<Name<Box<str>>>,
{
    move |token| {
        let result = AnchorRequestNode::new(name.into(), 0);
        let used_token = token.add_node(begin_mark, Node::AnchorRequest(result));
        Ok((used_token, output))
    }
}

/// Creates [`Data`] by combining the functions of this [module][`crate::data::make::combinator`].
/// 
/// Checks the existence of all requested anchors and the uniqueness of all created anchors.
///
/// Returns the same output returned by `f`.
///
/// *Note*: This is a simplified version of the [`make_document`].
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `f` Closure that creates a top node.
/// 
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use std::convert::Infallible;
///
/// let mark = Default::default();
/// let (data, _) = make::make::<_, Infallible, _>(mark, make::null(mark, ())).unwrap();
/// assert!(data.view().is_null())
/// ```
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
            let document = DocumentNode {
                node_index: index,
                anchors,
                ..Default::default()
            };
            (token, (document, output))
        })
    });

    match result {
        Ok((token, (document, output))) => {
            let _ = token.add_node(begin_mark, Node::Document(document));
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

/// Creates [`Data`] by combining the functions of this [module][`self`].
/// 
/// Checks the existence of all requested anchors and the uniqueness of all created anchors.
/// 
/// The top node is always Document, this is necessary for anchors to work correctly. 
/// `f` creates a child node of the document, which is commonly referred to as the top node.
/// 
/// Gives `f` one token, which not only gives but obliges to create exactly one node. 
/// This is guaranteed by the fact that when [`Token`] is used, it is consumed and if successful, [`UsedToken`][`super::maker::UsedToken`] is returned. 
/// And the closure signature obliges to return [`UsedToken`][`super::maker::UsedToken`] in case of success.
/// 
/// The token can be used by calling one of the functions of this [module][`self`] indirectly or directly. 
/// Some functions of this module issue additional tokens for creating child nodes.
/// 
/// In case of an unrecoverable error, an [`ErrorToken`][`super::maker::ErrorToken`] is formed and functions start deploying as fast as possible, 
/// when creating an [`ErrorToken`][`super::maker::ErrorToken`] the only thing you are allowed to do is return it from the function expanding the stack.
///
/// Returns the same output returned by `f`.
///
/// # Arguments
/// * `begin_mark` Node start mark.
/// * `path` Document path or document path in the document system analogy.
/// * `anchors` Closure that adds external named nodes (anchors).
/// * `f` Closure that adds a top node.
///
/// # Example
///
/// ```rust
/// use serde_ieml::data::make;
/// use serde_ieml::data::name::Name;
/// use std::{convert::Infallible, path::Path};
///
/// let mark = Default::default();
/// let (data, _) = make::make_document::<_, Infallible, _, _>(
///     mark, 
///     Path::new("test.ieml").to_path_buf(),
///     |token| token.add(mark, Name::new("anchor").unwrap(), make::null(mark, ())),
///     make::null(mark, ()),
/// ).unwrap();
///
/// let document_view = data.view().document().unwrap();
/// assert_eq!(document_view.path(), Path::new("test.ieml"));
/// 
/// let anchors = document_view.anchors().document_anchors();
/// assert_eq!(anchors.len(), 1);
/// assert!(anchors.get("anchor").unwrap().is_null());
/// ```
pub fn make_document<O, E, F, A>(
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

    match document(begin_mark, path, anchors, f)(Token::new(&mut maker)) {
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
    use super::super::super::{name::Name, node_type::NodeType};
    use super::*;
    use std::convert::Infallible;

    fn name(i: &str) -> Name<&str> {
        Name::new(i).unwrap()
    }

    #[test]
    fn test_null() {
        let begin_mark = Mark::default();
        let (data, _) = make::<_, Infallible, _>(begin_mark, null(begin_mark, ())).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_document().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Null);
    }

    #[test]
    fn test_raw() {
        let begin_mark = Mark::default();
        let (data, _) = make::<_, Infallible, _>(begin_mark, raw(begin_mark, (), "hello")).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_document().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Raw);
        assert_eq!(clear_view.raw().unwrap().raw(), "hello");
    }

    #[test]
    fn test_string() {
        let begin_mark = Mark::default();
        let (data, _) =
            make::<_, Infallible, _>(begin_mark, string(begin_mark, (), "hello")).unwrap();
        let view = data.view();
        let clear_view = view.clear_step_document().unwrap();

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
        let clear_view = view.clear_step_document().unwrap();

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
        let clear_view = view.clear_step_document().unwrap();

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
        let clear_view = view.clear_step_document().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Tagged);
        assert_eq!(view.tagged().unwrap().tag(), name("tag"));

        assert!(view.is_null());
    }

    #[test]
    fn test_document() {
        let begin_mark = Mark::default();
        let (data, _) = make::<_, Infallible, _>(begin_mark, {
            document(
                begin_mark,
                "dir/name.ieml".into(),
                |map_token| {
                    let (map_token, _) =
                        map_token.add(begin_mark, name("document-anchor"), null(begin_mark, ()))?;
                    Ok((map_token, ()))
                },
                raw(begin_mark, (), "hello"),
            )
        })
        .unwrap();
        let view = data.view();
        let clear_view = view.clear_step_document().unwrap();

        assert_eq!(clear_view.node_type(), NodeType::Document);
        assert_eq!(
            clear_view.document().unwrap().path(),
            PathBuf::from("dir/name.ieml").as_path()
        );

        let anchors = clear_view.document().unwrap().anchors().document_anchors();
        assert_eq!(anchors.len(), 1);
        assert!(anchors.contains_key("document-anchor"));

        assert!(clear_view.is_raw());
        assert_eq!(clear_view.raw().unwrap().raw(), "hello");
    }
}
