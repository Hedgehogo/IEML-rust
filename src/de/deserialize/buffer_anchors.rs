//! Trait definition [`BufferAnchors`]

use super::Result;
use crate::data::{
    error::{marked, CustomError},
    view::{analyse_anchors::AnalyseAnchors, View},
};
use serde::de;

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
    /// # Generic arguments
    /// * `V` Visitor for type of expected value through which the buffered value can be accessed. For example, it could be an id or a map key.
    ///
    /// # Arguments
    /// * `view` View from which the element will be deserialised if it does not exist.
    /// * `name` Anchor name, missing if no anchor is present.
    ///
    /// # Return value
    /// In Ok variant - a unique id, by which you can access the element.
    /// In Err variant - deserialisation error.
    fn entry<V: de::Visitor<'data>>(
        view: View<'data, Self>,
        name: Option<&'data str>,
        visitor: V,
    ) -> Result<V::Value>;
}

impl<'data> BufferAnchors<'data> for () {
    fn entry<V: de::Visitor<'data>>(
        view: View<'data, Self>,
        _name: Option<&'data str>,
        _visitor: V,
    ) -> Result<V::Value> {
        let custom_error = CustomError::new("anchor buffering is not supported".into());
        let deserialize = marked::DeserializeError::new(view.mark(), custom_error.into());
        Err(deserialize)
    }
}
