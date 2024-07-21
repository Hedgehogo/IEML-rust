use super::super::{data::Data, node::node::Node, view::anchors::Anchors};
use super::error::*;

fn init_step<E: std::error::Error + PartialEq + Eq>(
    data: &mut Data,
    file_index: usize,
    index: usize,
) -> Result<(), marked::Error<E>> {
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
                            return Err(marked::Error::new_with(
                                node.mark,
                                file.path.clone(),
                                ErrorKind::AnchorDoesntExist(i.name.clone()),
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

pub(super) fn init<E: std::error::Error + PartialEq + Eq>(
    data: &mut Data,
) -> Result<(), marked::Error<E>> {
    match &data.get(data.data.len() - 1).node {
        Node::File(i) => init_step(data, data.data.len() - 1, i.node_index),
        _ => Ok(()),
    }
}
