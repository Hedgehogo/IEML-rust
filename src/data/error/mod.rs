//! Definition of error types occurring during reading of IEML data structure

pub mod deserialize;
pub mod expected_names;
pub mod invalid_length;
pub mod invalid_type;
pub mod invalid_value;
pub mod missing_key;
pub mod unknown_key;
pub mod unknown_tag;

pub use crate::error::custom::CustomError;

pub use deserialize::DeserializeError;
pub use invalid_length::InvalidLengthError;
pub use invalid_type::InvalidTypeError;
pub use invalid_value::InvalidValueError;
pub use missing_key::MissingKeyError;
pub use unknown_key::UnknownKeyError;
pub use unknown_tag::UnknownTagError;

pub mod marked {
    pub use crate::error::marked::MarkedError;

    pub use super::deserialize::marked::DeserializeError;
    pub use super::invalid_length::marked::InvalidLengthError;
    pub use super::invalid_type::marked::InvalidTypeError;
    pub use super::invalid_value::marked::InvalidValueError;
    pub use super::missing_key::marked::MissingKeyError;
    pub use super::unknown_key::marked::UnknownKeyError;
    pub use super::unknown_tag::marked::UnknownTagError;
}
