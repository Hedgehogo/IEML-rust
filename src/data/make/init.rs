use super::super::{data::Data, node::node::Node, view::anchors::Anchors};
use super::error::*;
use std::error::Error;

pub(crate) fn init_step<E: Error + PartialEq + Eq>(
    data: &mut Data,
    file_index: usize,
    index: usize,
) -> Result<(), marked::MakeError<E>> {
    let mut node = std::mem::take(data.get_mut(index));
    match &mut node.node {
        Node::List(i) => {
            for i in i.data.iter() {
                init_step(data, file_index, *i)?;
            }
        }
        Node::Map(i) => {
            for (_, i) in i.data.iter() {
                init_step(data, file_index, *i)?;
            }
        }
        Node::Tagged(i) => init_step(data, file_index, i.node_index)?,
        Node::File(i) => i.parent = Some(file_index),
        Node::TakeAnchor(i) => init_step(data, file_index, i.node_index)?,
        Node::GetAnchor(i) => {
            let file_node = std::mem::take(data.get_mut(file_index));
            match &file_node.node {
                Node::File(file) => {
                    let anchors = Anchors::new(Default::default(), file, data, ());
                    match anchors.get_index(i.name.as_str()) {
                        Some(j) => i.node_index = j,
                        None => {
                            return Err(marked::MakeError::new(
                                node.mark,
                                MakeError::new(
                                    file.path.clone(),
                                    MakeErrorReason::AnchorDoesntExist(i.name.clone()),
                                ),
                            ))
                        }
                    };
                }
                _ => panic!("Incorrect document structure, the node is not a File."),
            }
            *data.get_mut(file_index) = file_node;
        }
        _ => {}
    }
    *data.get_mut(index) = node;
    if let Node::File(ref i) = data.get(index).node {
        let file_anchors = i.file_anchors.data.values().copied().collect::<Vec<_>>();
        init_step(data, index, i.node_index)?;
        for i in file_anchors {
            init_step(data, index, i)?;
        }
    }
    Ok(())
}

pub(crate) fn init<E: Error + PartialEq + Eq>(data: &mut Data) -> Result<(), marked::MakeError<E>> {
    match &data.get(data.data.len() - 1).node {
        Node::File(i) => init_step(data, data.data.len() - 1, i.node_index),
        _ => Ok(()),
    }
}
