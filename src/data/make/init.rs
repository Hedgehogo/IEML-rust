use super::super::{data::Data, node::node::Node, view::anchors::Anchors};
use super::error::*;

fn init_step<E: std::error::Error + PartialEq + Eq>(
    data: &mut Data,
    document_index: usize,
    index: usize,
) -> Result<(), marked::Error<E>> {
    let mut node = std::mem::take(data.get_mut(index));
    match &mut node.node {
        Node::List(i) => {
            for &i in i.data.iter() {
                init_step(data, document_index, i)?;
            }
        }
        Node::Map(i) => {
            for (_, &i) in i.data.iter() {
                init_step(data, document_index, i)?;
            }
        }
        Node::Tagged(i) => init_step(data, document_index, i.node_index)?,
        Node::Document(i) => i.parent = Some(document_index),
        Node::Anchor(i) => {
            if i.creation {
                init_step(data, document_index, i.node_index)?
            } else {
                let document_node = std::mem::take(data.get_mut(document_index));
                match &document_node.node {
                    Node::Document(document) => {
                        let anchors = Anchors::new(Default::default(), document, data, ());
                        match anchors.get_index(i.name.as_str()) {
                            Some(j) => i.node_index = j,
                            None => {
                                return Err(marked::Error::new_with(
                                    node.mark,
                                    document.path.clone(),
                                    ErrorKind::AnchorDoesntExist(i.name.clone()),
                                ))
                            }
                        };
                    }
                    _ => panic!("Incorrect document structure, the node is not a Document."),
                }
                *data.get_mut(document_index) = document_node;
            }
        }
        _ => {}
    }
    *data.get_mut(index) = node;
    if let Node::Document(ref i) = data.get(index).node {
        let document_anchors = i
            .document_anchors
            .data
            .values()
            .copied()
            .collect::<Vec<_>>();
        init_step(data, index, i.node_index)?;
        for i in document_anchors {
            init_step(data, index, i)?;
        }
    }
    Ok(())
}

pub(super) fn init<E: std::error::Error + PartialEq + Eq>(
    data: &mut Data,
) -> Result<(), marked::Error<E>> {
    match &data.get(data.data.len() - 1).node {
        Node::Document(i) => init_step(data, data.data.len() - 1, i.node_index),
        _ => Ok(()),
    }
}
