use super::{
    super::{
        data::Data,
        mark::Mark,
        name::Name,
        node::node::{MapNode, MarkedNode, Node},
    },
    error::{marked, ErrorKind, RateError},
};
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

pub(super) struct Maker {
    data: Data,
    anchors: MapNode,
    path: PathBuf,
}

impl Maker {
    pub(super) fn new(path: PathBuf) -> Self {
        Self {
            data: Default::default(),
            anchors: Default::default(),
            path,
        }
    }

    pub(super) fn data(self) -> Data {
        self.data
    }

    pub(super) fn path(&self) -> &Path {
        self.path.as_path()
    }
}

/// Token that allows adding exactly one direct child node.
pub struct Token<'maker> {
    maker: &'maker mut Maker,
}

impl<'maker> Token<'maker> {
    pub(super) fn new(maker: &'maker mut Maker) -> Self {
        Self { maker }
    }

    pub(super) fn add_node(self, mark: Mark, node: Node) -> UsedToken<'maker> {
        let data = &mut self.maker.data.data;
        data.insert(data.len(), MarkedNode::new(node, mark));
        UsedToken {
            index: data.len() - 1,
            maker: self.maker,
        }
    }

    pub(super) fn add_list(self) -> ListToken<'maker> {
        ListToken {
            result: Default::default(),
            maker: self.maker,
        }
    }

    pub(super) fn add_map(self) -> MapToken<'maker> {
        MapToken {
            result: Default::default(),
            maker: self.maker,
        }
    }

    pub(super) fn add_anchor<E>(
        self,
        mark: Mark,
        name: Name<Box<str>>,
        index: usize,
    ) -> Result<(Self, ()), (Self, marked::Error<E>)>
    where
        E: Error + PartialEq + Eq,
    {
        match self.maker.anchors.data.insert(name.clone(), index) {
            None => Ok((self, ())),
            Some(_) => {
                let path = PathBuf::from(self.maker.path());
                let kind = ErrorKind::AnchorAlreadyExist(name);
                let error = marked::Error::new_with(mark, path, kind);
                Err((self, error))
            }
        }
    }

    pub(super) fn child<F, R, E>(self, f: F) -> marked::ChildResult<'maker, R, E>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::ChildResult<'maker, R, E>,
    {
        let anchors = std::mem::take(&mut self.maker.anchors);
        match f(self) {
            Ok((token, result)) => {
                token.maker.anchors = anchors;
                Ok((token, result))
            }
            Err(error) => match error {
                RateError::Recoverable((token, error)) => {
                    token.maker.anchors = anchors;
                    Err(RateError::Recoverable((token, error)))
                }

                RateError::Unrecoverable((used_token, error)) => {
                    Err(RateError::Unrecoverable((used_token, error)))
                }
            },
        }
    }

    pub(super) fn anchors(self) -> (Self, MapNode) {
        let anchors = std::mem::take(&mut self.maker.anchors);
        (self, anchors)
    }

    /// Gets [`ErrorToken`].
    ///
    /// *Note*: Such a conversion is irreversible, the only thing you can do is throw the error above.
    pub fn error(self) -> ErrorToken<'maker> {
        ErrorToken { _maker: self.maker }
    }

    /// Adds one child node.
    ///
    /// *Note*: Function added for symmetry with [`ListToken`] and [`MapToken`].
    ///
    /// # Arguments
    /// * `f` Closure that adds a node.
    ///
    /// # Example
    ///
    /// ```rust
    /// use serde_ieml::data::make;
    /// use std::convert::Infallible;
    ///
    /// let mark = Default::default();
    /// let left = make::make::<_, Infallible, _>(mark, |token| make::null(mark, ())(token));
    /// let right = make::make::<_, Infallible, _>(mark, |token| token.add(make::null(mark, ())));
    /// assert_eq!(left, right)
    /// ```
    pub fn add<O, E, F>(self, f: F) -> marked::Result<'maker, O, E>
    where
        E: std::error::Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::Result<'maker, O, E>,
    {
        f(self)
    }
}

/// Token indicating that the node has been added.
pub struct UsedToken<'maker> {
    index: usize,
    maker: &'maker mut Maker,
}

impl<'maker> UsedToken<'maker> {
    pub(super) fn split(self) -> (Token<'maker>, usize) {
        (Token::new(self.maker), self.index)
    }

    /// Gets [`ErrorToken`].
    ///
    /// *Note*: Allows you to get an error if you have already added extra nodes. But only an unrecoverable one.
    pub fn error(self) -> ErrorToken<'maker> {
        ErrorToken { _maker: self.maker }
    }
}

/// Token indicating that an unrecoverable error has occurred.
pub struct ErrorToken<'maker> {
    _maker: &'maker mut Maker,
}

/// Token allowing to add any number of nodes to the list.
pub struct ListToken<'maker> {
    result: Vec<usize>,
    maker: &'maker mut Maker,
}

impl<'maker> ListToken<'maker> {
    pub(super) fn split(self) -> (Token<'maker>, Vec<usize>) {
        (Token::new(self.maker), self.result)
    }

    /// Gets [`ErrorToken`].
    ///
    /// *Note*: Such a conversion is irreversible, the only thing you can do is throw the error above.
    pub fn error(self) -> ErrorToken<'maker> {
        ErrorToken { _maker: self.maker }
    }

    /// Adds one child node, and returns the token back, allowing it to be reused.
    ///
    /// # Arguments
    /// * `f` Closure that adds a node.
    pub fn add<O, E, F>(mut self, f: F) -> marked::ListResult<'maker, O, E>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::Result<'maker, O, E>,
    {
        match f(Token::new(self.maker)) {
            Ok((used_token, output)) => {
                self.result.push(used_token.index);
                self.maker = used_token.maker;
                Ok((self, output))
            }

            Err(error) => match error {
                RateError::Recoverable((token, error)) => {
                    self.maker = token.maker;
                    Err(RateError::Recoverable((self, error)))
                }

                RateError::Unrecoverable((used_token, error)) => {
                    Err(RateError::Unrecoverable((used_token, error)))
                }
            },
        }
    }
}

/// Token allowing to add any number of nodes to the map.
pub struct MapToken<'maker> {
    result: HashMap<Name<Box<str>>, usize>,
    maker: &'maker mut Maker,
}

impl<'maker> MapToken<'maker> {
    pub(super) fn split(self) -> (Token<'maker>, HashMap<Name<Box<str>>, usize>) {
        (Token::new(self.maker), self.result)
    }

    /// Gets [`ErrorToken`].
    ///
    /// *Note*: Such a conversion is irreversible, the only thing you can do is throw the error above.
    pub fn error(self) -> ErrorToken<'maker> {
        ErrorToken { _maker: self.maker }
    }

    /// Adds one child node, and returns the token back, allowing it to be reused.
    ///
    /// # Arguments
    /// * `mark` Mark of the beginning of the key.
    /// * `key` Key by which the added node will be accessed.
    /// * `f` Closure that adds a node.
    pub fn add<O, E, F, S>(mut self, mark: Mark, key: S, f: F) -> marked::MapResult<'maker, O, E>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::Result<'maker, O, E>,
        S: Into<Name<Box<str>>>,
    {
        match f(Token::new(self.maker)) {
            Ok((used_token, output)) => {
                self.maker = used_token.maker;
                match self.result.insert(key.into(), used_token.index) {
                    None => Ok((self, output)),

                    Some(_) => {
                        let path = PathBuf::from(self.maker.path());
                        let error = marked::Error::new_with(mark, path, ErrorKind::RepeatedKey);
                        Err(RateError::Recoverable((self, error)))
                    }
                }
            }

            Err(error) => match error {
                RateError::Recoverable((token, error)) => {
                    self.maker = token.maker;
                    Err(RateError::Recoverable((self, error)))
                }

                RateError::Unrecoverable((used_token, error)) => {
                    Err(RateError::Unrecoverable((used_token, error)))
                }
            },
        }
    }
}
