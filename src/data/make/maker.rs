use super::{
    super::{
        data::Data,
        mark::Mark,
        name::Name,
        node::node::{MapNode, MarkedNode, Node},
    },
    error::{marked, MakeErrorReason::RepeatedKey},
};
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

pub struct Maker {
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

    pub(super) fn child<'maker, F, R>(&'maker mut self, f: F) -> (&'maker mut Maker, R)
    where
        F: FnOnce(&'maker mut Maker) -> (&'maker mut Maker, R),
    {
        let anchors = std::mem::take(&mut self.anchors);
        let (maker, result) = f(self);
        maker.anchors = anchors;
        (maker, result)
    }

    pub(super) fn add(&mut self, mark: Mark, node: Node) -> UsedToken {
        self.data
            .data
            .insert(self.data.data.len(), MarkedNode::new(node, mark));
        UsedToken::new(self.last(), self)
    }

    pub(super) fn last(&self) -> usize {
        self.data.data.len() - 1
    }

    pub(super) fn add_anchor(&mut self, name: Name, index: usize) -> Option<()> {
        self.anchors
            .data
            .insert(name, index)
            .is_none()
            .then_some(())
    }

    pub(super) fn anchors(&mut self) -> &mut MapNode {
        &mut self.anchors
    }

    pub(super) fn data(self) -> Data {
        self.data
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }
}

pub struct UsedToken<'maker> {
    pub(super) index: usize,
    pub(super) maker: &'maker mut Maker,
}

impl<'maker> UsedToken<'maker> {
    fn new(index: usize, maker: &'maker mut Maker) -> Self {
        Self { index, maker }
    }
}

pub struct Token<'maker> {
    pub(super) maker: &'maker mut Maker,
}

impl<'maker> Token<'maker> {
    pub(super) fn new(maker: &'maker mut Maker) -> Self {
        Self { maker }
    }

    pub fn add<O, E, F>(self, f: F) -> marked::MakeResult<'maker, O, E>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token<'maker>) -> marked::MakeResult<'maker, O, E>,
    {
        f(self)
    }
}

pub struct ListToken<'maker> {
    pub(super) result: Vec<usize>,
    pub(super) maker: &'maker mut Maker,
}

impl<'maker> ListToken<'maker> {
    pub(super) fn new(maker: &'maker mut Maker) -> Self {
        Self {
            result: Default::default(),
            maker,
        }
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
    pub(super) result: HashMap<Name, usize>,
    pub(super) maker: &'maker mut Maker,
}

impl<'maker> MapToken<'maker> {
    pub(super) fn new(maker: &'maker mut Maker) -> Self {
        Self {
            result: Default::default(),
            maker,
        }
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
