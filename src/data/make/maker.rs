use super::{
    super::{
        data::Data,
        mark::Mark,
        node::node::{MapNode, MarkedNode, Node},
    },
    error::{marked, MakeErrorReason},
};
use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

pub struct Token {
    _field: (),
}

impl Token {
    pub(super) fn new() -> Self {
        Self { _field: () }
    }
}

pub struct Added {
    index: usize,
}

impl Added {
    fn new(index: usize) -> Self {
        Self { index }
    }

    pub(super) fn index(self) -> usize {
        self.index
    }
}

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

    pub(super) fn child<F: FnOnce(&mut Maker) -> R, R>(&mut self, f: F) -> R {
        let anchors = std::mem::take(&mut self.anchors);
        let result = f(self);
        self.anchors = anchors;
        result
    }

    pub(super) fn add(&mut self, mark: Mark, _token: Token, node: Node) -> Added {
        self.data
            .data
            .insert(self.data.data.len(), MarkedNode::new(node, mark));
        Added::new(self.last())
    }

    pub(super) fn last(&self) -> usize {
        self.data.data.len() - 1
    }

    pub(super) fn add_anchor(&mut self, name: String, index: usize) -> Option<()> {
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

    pub fn add<O, E, F>(&mut self, f: F) -> Result<O, marked::MakeError<E>>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    {
        match f(Token::new(), self.maker) {
            Ok((added, output)) => {
                self.result.push(added.index());
                Ok(output)
            }
            Err((_token, error)) => Err(error),
        }
    }
}

pub struct MapToken<'maker> {
    pub(super) result: HashMap<String, usize>,
    pub(super) maker: &'maker mut Maker,
}

impl<'maker> MapToken<'maker> {
    pub(super) fn new(maker: &'maker mut Maker) -> Self {
        Self {
            result: Default::default(),
            maker,
        }
    }

    pub fn add<O, E, F>(&mut self, mark: Mark, key: String, f: F) -> Result<O, marked::MakeError<E>>
    where
        E: Error + PartialEq + Eq,
        F: FnOnce(Token, &mut Maker) -> marked::MakeResult<O, E>,
    {
        match f(Token::new(), self.maker) {
            Ok((added, output)) => match self.result.insert(key, added.index()) {
                None => Ok(output),
                Some(_) => Err(marked::MakeError::new_with(
                    mark,
                    self.maker.path(),
                    MakeErrorReason::RepeatedKey,
                )),
            },
            Err((_token, error)) => Err(error),
        }
    }
}
