//! Type definition [`InvalidValueError`]

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

/// Deserialisation error type containing a child error - the reason.
#[derive(PartialEq, Eq, Debug)]
pub struct InvalidValueError<E> {
    expected: String,
    reason: Option<Box<marked::DeserializeError<E>>>,
}

impl<E> InvalidValueError<E> {
    pub fn new(expected: String, reason: Option<Box<marked::DeserializeError<E>>>) -> Self {
        Self { expected, reason }
    }

    pub fn expected(&self) -> &str {
        &self.expected
    }

    pub fn reason(&self) -> &Option<Box<marked::DeserializeError<E>>> {
        &self.reason
    }
}

impl<E: Display> Display for InvalidValueError<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            Some(i) => write!(
                f,
                "{}\nnote: when trying to receive {}",
                i.as_ref(),
                self.expected()
            ),
            None => write!(f, "error: expected {}", self.expected()),
        }
    }
}

impl<E: Error> Error for InvalidValueError<E> {}

pub mod marked {
    use crate::error::marked::MarkedError;

    pub(super) use super::super::deserialize::marked::DeserializeError;

    pub type InvalidValueError<E> = MarkedError<super::InvalidValueError<E>>;
}

#[cfg(test)]
mod tests {
    use super::super::super::{mark::Mark, node_type::NodeType};
    use super::super::{CustomError, InvalidTypeError};
    use super::*;

    #[test]
    fn test_display_no_reason() {
        let expected = "integer in the range from -2^31 to 2^31 - 1";
        let error = InvalidValueError::<CustomError>::new(expected.into(), None);
        assert_eq!(
            error.to_string(),
            "error: expected integer in the range from -2^31 to 2^31 - 1"
        );
    }

    #[test]
    fn test_display_reason() {
        let expected = &[NodeType::Tagged, NodeType::Anchor, NodeType::Document] as &[_];
        let invalid_type = InvalidTypeError::new(NodeType::Raw, expected);
        let deserialize = marked::DeserializeError::new(Mark::new(1, 1), invalid_type.into());

        let expected = "optional value";
        let error =
            InvalidValueError::<CustomError>::new(expected.into(), Some(Box::new(deserialize)));

        assert_eq!(
            error.to_string(),
            "error: value of unexpected type number, boolean, raw data, expected types: tagged, anchor, document
 --> 1:1
note: when trying to receive optional value"
        );
    }
}
