//! Trait definition [`BufferAnchors`]

use super::super::error::{marked, CustomError};
use super::{analyse_anchors::AnalyseAnchors, AnchorView};

type Error = marked::DeserializeError<CustomError>;

/// Trait allowing to buffer values available by anchor without duplication.
/// 
/// The bufferiser is supposed to be a reference data structure to some buffer.
/// 
/// For example, this allows you to deserialize anchor data only once, and then only take a reference to already created structures.
/// 
/// Note: This trait is designed to bypass serde restrictions
pub trait BufferAnchors<'data>: AnalyseAnchors<'data> {
    /// Inserts or gets an element from the buffer.
    ///
    /// # Arguments
    /// * `ty` The type of item to be inserted or retrieved, in a string can be for example its name or id.
    /// * `view` View the anchor from which the element will be deserialised if it does not exist.
    ///
    /// # Return value
    /// In Ok variant - a unique id, by which you can access the element.
    /// In Err variant - deserialisation error.
    fn entry(&self, ty: &'static str, view: AnchorView<'data, Self>) -> Result<usize, Error>;
}

impl<'data> BufferAnchors<'data> for () {
    fn entry(&self, _ty: &str, view: AnchorView<'data, Self>) -> Result<usize, Error> {
        let custom_error = CustomError::new("anchor buffering is not supported".into());
        let deserialize = Error::new(view.mark(), custom_error.into());
        Err(deserialize)
    }
}
