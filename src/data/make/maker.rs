use super::{
    super::{
        data::Data,
        mark::Mark,
        name::Name,
        node::node::{MapNode, MarkedNode, Node},
    },
    error::{
        marked,
        MakeErrorReason::{AnchorAlreadyExist, RepeatedKey},
    },
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

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

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
        name: Name,
        index: usize,
    ) -> Result<(Self, ()), (Self, marked::MakeError<E>)>
    where
        E: Error + PartialEq + Eq,
    {
        match self.maker.anchors.data.insert(name.clone(), index) {
            None => Ok((self, ())),
            Some(_) => {
                let path = PathBuf::from(self.maker.path());
                let reason = AnchorAlreadyExist(name);
                let error = marked::MakeError::new_with(mark, path, reason);
                Err((self, error))
            }
        }
    }

    pub(super) fn child<F, R>(self, f: F) -> (Token<'maker>, R)
    where
        F: FnOnce(Token<'maker>) -> (Token<'maker>, R),
    {
        let anchors = std::mem::take(&mut self.maker.anchors);
        let (token, result) = f(self);
        token.maker.anchors = anchors;
        (token, result)
    }

    pub(super) fn anchors(self) -> (Self, MapNode) {
        let anchors = std::mem::take(&mut self.maker.anchors);
        (self, anchors)
    }

    pub fn add<O, E, F>(self, f: F) -> marked::MakeResult<'maker, O, E>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::MakeResult<'maker, O, E>,
    {
        f(self)
    }
}

pub struct UsedToken<'maker> {
    index: usize,
    maker: &'maker mut Maker,
}

impl<'maker> UsedToken<'maker> {
    pub(super) fn split(self) -> (Token<'maker>, usize) {
        (Token::new(self.maker), self.index)
    }
}

pub struct ListToken<'maker> {
    result: Vec<usize>,
    maker: &'maker mut Maker,
}

impl<'maker> ListToken<'maker> {
    pub(super) fn split(self) -> (Token<'maker>, Vec<usize>) {
        (Token::new(self.maker), self.result)
    }

    pub fn add<O, E, F>(mut self, f: F) -> marked::MakeListResult<'maker, O, E>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::MakeResult<'maker, O, E>,
    {
        match f(Token::new(self.maker)) {
            Ok((used_token, output)) => {
                self.result.push(used_token.index);
                self.maker = used_token.maker;
                Ok((self, output))
            }
            Err((token, error)) => {
                self.maker = token.maker;
                Err((self, error))
            }
        }
    }
}

pub struct MapToken<'maker> {
    result: HashMap<Name, usize>,
    maker: &'maker mut Maker,
}

impl<'maker> MapToken<'maker> {
    pub(super) fn split(self) -> (Token<'maker>, HashMap<Name, usize>) {
        (Token::new(self.maker), self.result)
    }

    pub fn add<O, E, F, S>(
        mut self,
        mark: Mark,
        key: S,
        f: F,
    ) -> marked::MakeMapResult<'maker, O, E>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::MakeResult<'maker, O, E>,
        S: Into<Name>,
    {
        match f(Token::new(self.maker)) {
            Ok((used_token, output)) => {
                self.maker = used_token.maker;
                match self.result.insert(key.into(), used_token.index) {
                    None => Ok((self, output)),
                    Some(_) => {
                        let path = PathBuf::from(self.maker.path());
                        let error = marked::MakeError::new_with(mark, path, RepeatedKey);
                        Err((self, error))
                    }
                }
            }
            Err((token, error)) => {
                self.maker = token.maker;
                Err((self, error))
            }
        }
    }
}
