use std::{
    convert::Infallible,
    error::Error,
};
use crate::data::cell::MarkedDataCell;
use super::super::{
    cell::{DataCell, Data},
    node::anchors::Anchors,
};
use super::error::*;

pub(crate) fn init_step<E: Error + PartialEq + Eq>(data: &mut Data, file_index: usize, index: usize) -> Result<(), marked::MakeError<E>> {
    let mut cell = std::mem::take(data.get_mut(index));
    match &mut cell.cell {
        DataCell::List(i) => for i in i {
            init_step(data, file_index, *i)?;
        }
        DataCell::Map(i) => for (_, i) in i {
            init_step(data, file_index, *i)?;
        }
        DataCell::Tag(i) => init_step(data, file_index, i.cell_index)?,
        DataCell::File(i) => i.parent = Some(file_index),
        DataCell::TakeAnchor(i) => init_step(data, file_index, i.cell_index)?,
        DataCell::GetAnchor(i) => {
            let file_cell = std::mem::take(data.get_mut(file_index));
            match &file_cell.cell {
                DataCell::File(file) => {
                    let anchors = Anchors::<Infallible>::new(file, data);
                    match anchors.get_index(i.name.as_str()) {
                        Some(j) => i.cell_index = j,
                        None => return Err(marked::MakeError::new(cell.mark, MakeError::new(file.path.clone(), MakeErrorReason::AnchorDoesntExist(i.name.clone())))),
                    };
                },
                _ => panic!("Incorrect document structure, the cell is not a File."),
            }
            *data.get_mut(file_index) = file_cell;
        }
        _ => {}
    }
    *data.get_mut(index) = cell;
    if let DataCell::File(ref i) = data.get(index).cell {
        let file_anchors = i.file_anchors.values().copied().collect::<Vec<_>>();
        init_step(data, index, i.cell_index)?;
        for i in file_anchors {
            init_step(data, index, i)?;
        }
    }
    Ok(())
}

pub(crate) fn init<E: Error + PartialEq + Eq>(data: &mut Data) -> Result<(), marked::MakeError<E>> {
    match &data.get(data.index).cell {
        DataCell::File(i) => {
            init_step(data, data.index, i.cell_index)
        }
        _ => Ok(())
    }
}