//! Definition of types that can be used to buffer a deserialised value by anchor

use serde::{de, Deserialize};
use std::marker::PhantomData;

pub struct AnchorVisitor<T> {
    phantom: PhantomData<T>,
}

impl<T> AnchorVisitor<T> {
    fn new() -> Self {
        AnchorVisitor {
            phantom: PhantomData,
        }
    }
}

impl<'de, T> de::Visitor<'de> for AnchorVisitor<T> {
    type Value = AnchorId<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "anchor")
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_option(self)
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E> {
        Ok(Self::Value::new(value as usize))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnchorId<T> {
    pub value: usize,
    phantom: PhantomData<T>,
}

impl<T> AnchorId<T> {
    fn new(value: usize) -> Self {
        AnchorId {
            value,
            phantom: PhantomData,
        }
    }
}

impl<'de, T> Deserialize<'de> for AnchorId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let visitor = AnchorVisitor::<T>::new();
        deserializer.deserialize_newtype_struct("@", visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::super::error::marked;
    use super::super::{
        analyse_anchors::AnalyseAnchors, buffer_anchors::BufferAnchors, deserializer::Result,
        view::View,
    };
    use crate::from_source;
    use serde::Deserialize;
    use std::{any::type_name, cell::RefCell, collections::hash_map::HashMap};

    type TestBuffer = RefCell<HashMap<usize, String>>;

    #[derive(Clone, Copy)]
    struct TestBufferiser<'buffer> {
        buffer: &'buffer TestBuffer,
    }

    impl<'buffer> TestBufferiser<'buffer> {
        fn new(buffer: &'buffer TestBuffer) -> Self {
            Self { buffer }
        }
    }

    impl<'data, 'buffer> AnalyseAnchors<'data> for TestBufferiser<'buffer> {
        fn child(&self, _path: &'data std::path::Path) -> Self {
            self.clone()
        }

        fn parent(&self) -> Option<Self> {
            None
        }
    }

    impl<'data, 'buffer> BufferAnchors<'data> for TestBufferiser<'buffer> {
        fn entry<V: de::Visitor<'data>>(
            view: View<'data, Self>,
            _name: Option<&'data str>,
            visitor: V,
        ) -> Result<V::Value> {
            if type_name::<V::Value>() == type_name::<AnchorId<String>>() {
                let buffer = view.anchor_analyser().buffer;
                let id = view.id();
                if !buffer.borrow().contains_key(&id) {
                    let result: String = String::deserialize(view)?;
                    buffer.borrow_mut().insert(id, result);
                }
                visitor.visit_u128(id as u128)
            } else {
                let expected = "string".into();
                let error = marked::MarkedError::new_invalid_value(view.mark(), expected, None);
                Err(error)
            }
        }
    }

    #[test]
    fn test() {
        let buffer = TestBuffer::new(HashMap::new());
        let bufferiser = TestBufferiser::new(&buffer);
        let data = from_source("@anchor: > Hello").unwrap();
        let view = data.view_with_analyse(bufferiser);
        let result = AnchorId::<String>::deserialize(view).unwrap();

        assert_eq!(buffer.borrow().get(&result.value), Some(&"Hello".into()));
    }

    #[test]
    fn test_multi() {
        let input = r#"
- @anchor: > Value
- @anchor
- > No anchor
"#;

        let buffer = TestBuffer::new(HashMap::new());
        let bufferiser = TestBufferiser::new(&buffer);
        let data = from_source(input).unwrap();
        let view = data.view_with_analyse(bufferiser);
        let result = <[AnchorId<String>; 3]>::deserialize(view).unwrap();

        assert_eq!(result[0], result[1]);

        let borrowed = buffer.borrow();
        assert_eq!(borrowed.len(), 2);
        assert_eq!(borrowed.get(&result[0].value), Some(&"Value".into()));
        assert_eq!(borrowed.get(&result[1].value), Some(&"Value".into()));
        assert_eq!(borrowed.get(&result[2].value), Some(&"No anchor".into()));
    }
}
