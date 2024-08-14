//! Type definition [`DeserializeError`]

use super::*;

use std::{error::Error, fmt};

/// General type of deserialisation error
#[derive(PartialEq, Eq, Debug)]
pub enum DeserializeError<E> {
    /// Occurs when the reader receives a different type than expected.
    InvalidType(InvalidTypeError),
    /// Occurs when the reader receives a value of the right type but wrong for another reason.
    InvalidValue(InvalidValueError<E>),
    /// Occurs when reading a list or map, but the input data contains too many or too few elements.
    InvalidLength(InvalidLengthError),
    /// Occurs when reading a tagged node, but the tag does not match any of the expected ones.
    UnknownTag(UnknownTagError),
    /// Occurs when reading a map, but the input data contains the extra key.
    UnknownKey(UnknownKeyError),
    /// Occurs when reading a map, but the input data does not contain the requested key.
    MissingKey(MissingKeyError),
    /// Additional error type for the possibility of extending this type
    Custom(E),
}

impl<E: fmt::Display> fmt::Display for DeserializeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::InvalidType(i) => write!(f, "{}", i),
            DeserializeError::InvalidValue(i) => write!(f, "{}", i),
            DeserializeError::InvalidLength(i) => write!(f, "{}", i),
            DeserializeError::UnknownTag(i) => write!(f, "{}", i),
            DeserializeError::UnknownKey(i) => write!(f, "{}", i),
            DeserializeError::MissingKey(i) => write!(f, "{}", i),
            DeserializeError::Custom(i) => write!(f, "{}", i),
        }
    }
}

impl<E: fmt::Debug + fmt::Display> Error for DeserializeError<E> {}

impl<E> From<InvalidTypeError> for DeserializeError<E> {
    fn from(value: InvalidTypeError) -> Self {
        DeserializeError::InvalidType(value)
    }
}

impl<E> From<InvalidValueError<E>> for DeserializeError<E> {
    fn from(value: InvalidValueError<E>) -> Self {
        DeserializeError::InvalidValue(value)
    }
}

impl<E> From<InvalidLengthError> for DeserializeError<E> {
    fn from(value: InvalidLengthError) -> Self {
        DeserializeError::InvalidLength(value)
    }
}

impl<E> From<UnknownTagError> for DeserializeError<E> {
    fn from(value: UnknownTagError) -> Self {
        DeserializeError::UnknownTag(value)
    }
}

impl<E> From<UnknownKeyError> for DeserializeError<E> {
    fn from(value: UnknownKeyError) -> Self {
        DeserializeError::UnknownKey(value)
    }
}

impl<E> From<MissingKeyError> for DeserializeError<E> {
    fn from(value: MissingKeyError) -> Self {
        DeserializeError::MissingKey(value)
    }
}

impl From<CustomError> for DeserializeError<CustomError> {
    fn from(value: CustomError) -> Self {
        DeserializeError::Custom(value)
    }
}

pub mod marked {
    use super::super::super::{mark::Mark, node_type::NodeType};
    use super::super::{expected::Expected, marked::*};
    use crate::error::{custom::marked::CustomError, marked::MarkedError};
    use serde::de;
    use std::fmt;

    pub type DeserializeError<E> = MarkedError<super::DeserializeError<E>>;

    impl<E> DeserializeError<E> {
        pub fn new_invalid_type(
            mark: Mark,
            unexpected_type: NodeType,
            expected_types: impl Into<Expected<NodeType>>,
        ) -> Self {
            Self::new(
                mark,
                super::InvalidTypeError::new(unexpected_type, expected_types).into(),
            )
        }

        pub fn new_invalid_value(
            mark: Mark,
            expected: String,
            reason: Option<Box<DeserializeError<E>>>,
        ) -> Self {
            Self::new(mark, super::InvalidValueError::new(expected, reason).into())
        }

        pub fn new_invalid_length(mark: Mark, length: usize) -> Self {
            Self::new(mark, super::InvalidLengthError::new(length).into())
        }

        pub fn new_unknown_tag(
            mark: Mark,
            unexpected_tag: String,
            expected_tags: impl Into<Expected<&'static str>>,
        ) -> Self {
            Self::new(
                mark,
                super::UnknownTagError::new(unexpected_tag, expected_tags).into(),
            )
        }

        pub fn new_unknown_key(
            mark: Mark,
            unexpected_key: String,
            expected_keys: impl Into<Expected<&'static str>>,
        ) -> Self {
            Self::new(
                mark,
                super::UnknownKeyError::new(unexpected_key, expected_keys).into(),
            )
        }

        pub fn new_missing_key(mark: Mark, expected_key: String) -> Self {
            Self::new(mark, super::MissingKeyError::new(expected_key).into())
        }
    }

    impl<E> From<InvalidTypeError> for DeserializeError<E> {
        fn from(value: InvalidTypeError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::InvalidType(value.data))
        }
    }

    impl<E> From<InvalidValueError<E>> for DeserializeError<E> {
        fn from(value: InvalidValueError<E>) -> Self {
            MarkedError::new(
                value.mark,
                super::DeserializeError::InvalidValue(value.data),
            )
        }
    }

    impl<E> From<InvalidLengthError> for DeserializeError<E> {
        fn from(value: InvalidLengthError) -> Self {
            MarkedError::new(
                value.mark,
                super::DeserializeError::InvalidLength(value.data),
            )
        }
    }

    impl<E> From<UnknownTagError> for DeserializeError<E> {
        fn from(value: UnknownTagError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::UnknownTag(value.data))
        }
    }

    impl<E> From<UnknownKeyError> for DeserializeError<E> {
        fn from(value: UnknownKeyError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::UnknownKey(value.data))
        }
    }

    impl<E> From<MissingKeyError> for DeserializeError<E> {
        fn from(value: MissingKeyError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::MissingKey(value.data))
        }
    }

    impl From<CustomError> for DeserializeError<super::CustomError> {
        fn from(value: CustomError) -> Self {
            MarkedError::new(value.mark, super::DeserializeError::Custom(value.data))
        }
    }

    impl de::Error for DeserializeError<super::CustomError> {
        fn custom<T>(msg: T) -> Self
        where
            T: fmt::Display,
        {
            let custom = CustomError::from(msg.to_string());
            MarkedError::new(custom.mark, super::DeserializeError::Custom(custom.data))
        }

        fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
            let expected = format!("{}", exp);
            let invelid_type = super::InvalidTypeError::new(unexp.into(), &[] as &_);
            let deserialize = DeserializeError::new(Default::default(), invelid_type.into());
            let invalid_value =
                super::InvalidValueError::new(expected, Some(Box::new(deserialize)));
            DeserializeError::new(Default::default(), invalid_value.into())
        }

        fn invalid_value(_unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
            let expected = format!("{}", exp);
            let invalid_value = super::InvalidValueError::new(expected, None);
            DeserializeError::new(Default::default(), invalid_value.into())
        }

        fn invalid_length(len: usize, exp: &dyn de::Expected) -> Self {
            let expected = format!("{}", exp);
            let invelid_length = super::InvalidLengthError::new(len);
            let deserialize = DeserializeError::new(Default::default(), invelid_length.into());
            let invalid_value =
                super::InvalidValueError::new(expected, Some(Box::new(deserialize)));
            DeserializeError::new(Default::default(), invalid_value.into())
        }

        fn unknown_variant(variant: &str, expected: &'static [&'static str]) -> Self {
            let unknown_tag = super::UnknownTagError::new(variant.into(), expected);
            DeserializeError::new(Default::default(), unknown_tag.into())
        }

        fn unknown_field(field: &str, expected: &'static [&'static str]) -> Self {
            let unknown_key = super::UnknownKeyError::new(field.into(), expected);
            DeserializeError::new(Default::default(), unknown_key.into())
        }

        fn missing_field(field: &'static str) -> Self {
            let missing_key = super::MissingKeyError::new(field.into());
            DeserializeError::new(Default::default(), missing_key.into())
        }
    }
}
