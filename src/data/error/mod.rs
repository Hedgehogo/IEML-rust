//! Definition of error types occurring during reading of IEML data structure

pub mod deserialize;
pub mod invalid_length;
pub mod invalid_type;
pub mod invalid_value;
pub mod missing_key;

pub use deserialize::DeserializeError;
pub use invalid_length::InvalidLengthError;
pub use invalid_type::InvalidTypeError;
pub use invalid_value::InvalidValueError;
pub use missing_key::MissingKeyError;

pub mod marked {
    pub use crate::error::marked::MarkedError;

    pub type DeserializeError<E> = super::deserialize::marked::DeserializeError<E>;
    pub type InvalidTypeError = super::invalid_type::marked::InvalidTypeError;
    pub type InvalidValueError<E> = super::invalid_value::marked::InvalidValueError<E>;
    pub type InvalidLengthError = super::invalid_length::marked::InvalidLengthError;
    pub type MissingKeyError = super::missing_key::marked::MissingKeyError;
}
