use super::super::{
    data::Data,
    node::node::{DocumentNode, MarkedNode, Node},
};
use super::error::*;

struct Anchors<'data> {
    document: &'data DocumentNode,
    paremt: Option<&'data Anchors<'data>>,
}

impl<'data> Anchors<'data> {
    fn new(document: &'data DocumentNode, paremt: Option<&'data Anchors<'data>>) -> Self {
        Self { document, paremt }
    }

    fn get(&self, name: &'data str) -> Option<usize> {
        self.document.anchors.data.get(name).copied().or_else(|| {
            self.document
                .document_anchors
                .data
                .get(name)
                .copied()
                .or_else(|| self.paremt.and_then(|i| i.get(name)))
        })
    }
}

fn split_last(slice: &mut [MarkedNode]) -> (&mut [MarkedNode], &mut MarkedNode) {
    let (result, last) = slice.split_at_mut(slice.len() - 1);
    (result, &mut last[0])
}

fn split_index(slice: &mut [MarkedNode], index: usize) -> &mut [MarkedNode] {
    let (slice, _) = slice.split_at_mut(index + 1);
    slice
}

fn init_step<E: std::error::Error + PartialEq + Eq>(
    slice: &mut [MarkedNode],
    anchors: &Anchors,
) -> Result<(), marked::Error<E>> {
    let (slice, last) = split_last(slice);

    match &mut last.node {
        Node::List(i) => {
            for &index in i.data.iter() {
                init_step(split_index(slice, index), anchors)?;
            }
        }

        Node::Map(i) => {
            for (_, &index) in i.data.iter() {
                init_step(split_index(slice, index), anchors)?;
            }
        }

        Node::Tagged(i) => init_step(split_index(slice, i.node_index), anchors)?,

        Node::Document(i) => {
            for (_, &index) in i.document_anchors.data.iter() {
                init_step(split_index(slice, index), anchors)?;
            }

            let anchors = Anchors::new(&*i, Some(anchors));
            init_step(split_index(slice, i.node_index), &anchors)?;
        }

        Node::Anchor(i) => {
            if i.creation {
                init_step(split_index(slice, i.node_index), anchors)?;
            } else {
                match anchors.get(i.name.as_str()) {
                    Some(index) => i.node_index = index,

                    None => {
                        let mark = last.mark;
                        let path = anchors.document.path.clone();
                        let error_kind = ErrorKind::AnchorDoesntExist(i.name.clone());
                        return Err(marked::Error::new_with(mark, path, error_kind));
                    }
                }
            }
        }

        _ => {}
    };

    Ok(())
}

pub(super) fn init<E: std::error::Error + PartialEq + Eq>(
    data: &mut Data,
) -> Result<(), marked::Error<E>> {
    let slice = data.data.as_mut_slice();
    let (slice, last) = split_last(slice);
    
    match &last.node {
        Node::Document(i) => {
            let anchors = Anchors::new(i, None);
            init_step(slice, &anchors)
        }

        _ => Ok(()),
    }
}
