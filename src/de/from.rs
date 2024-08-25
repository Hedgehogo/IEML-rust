//! Deserialize an IEML input to a Rust data structure.

use super::{
    deserialize::buffer_anchors::BufferAnchors,
    parse::{parse_with::*, read_source::ReadSource},
};
use crate::{
    data::make,
    error::{common::marked, custom::CustomError},
};
use serde::de;

pub type Error = marked::CommonError<CustomError>;
pub type Result<T> = std::result::Result<T, Error>;

/// Creates an object of type T using a source reader (implements [`ReadSource`]).
/// Accepts a closure that generates external anchors that will be available in the document.
/// Accepts a bufferiser, to add anchor-like structures to the buffer.
///
/// # Arguments
/// * `reader` The source reader.
/// * `anchors` The closure that generates external anchors.
///   (More information in [`make_document`][crate::data::make::combinator::make_document])
/// * `anchor_bufferiser` An object which, or copies of which, allows anchor-like structures to be added to the buffer.
///
/// # Generic arguments
/// * `T` Type of expected return value.
/// * `R` The type of source reader, in particular it can be a [`str`] or a [`Path`][std::path::Path].
/// * `A` Type of closure that generates external anchors.
/// * `B` The type of anchor bufferiser, in particular it can be `()`, then an error will always be returned when trying to buffer anchors.
///
/// Returns either a value of type T or a common error for deserialisation from both IEML input to IEML data
/// structure and from IEML data structure to Rust data structure.
///
/// *Note*:
/// The source reader can access the document system.
/// If you need to borrow data (e.g. deserialise `&str`), use [`parse_with_reader`], then
/// [`Data::view`][crate::data::data::Data::view] and [`Deserialize::deserialize`][de::Deserialize::deserialize].
///
/// # Example
///
/// ```rust
/// use serde_ieml::from_source_advanced;
/// use serde_ieml::data::name::Name;
/// use serde_ieml::data::make;
///
/// let value = from_source_advanced::<String, _, _, _>(
///     "@anchor",
///     |token| {
///         let (token, _) = token.add(
///             Default::default(),
///             Name::new("anchor").unwrap(),
///             make::string(Default::default(), (), "hello"),
///         )?;
///         Ok(token)
///     },
///     ()
/// );
///
/// assert_eq!(value, Ok("hello".into()));
/// ```
pub fn from_source_advanced<T, R, A, B>(reader: &R, anchors: A, anchor_bufferiser: B) -> Result<T>
where
    T: for<'data> de::Deserialize<'data>,
    R: ReadSource + ?Sized,
    A: FnOnce(make::MapToken) -> AnchorsResult,
    B: for<'data> BufferAnchors<'data>,
{
    let data = parse_with_reader_and_anchors(reader, anchors)?;
    let result = T::deserialize(data.deserializer_with_bufferiser(anchor_bufferiser))?;
    Ok(result)
}

/// Creates an object of type T using a source reader (implements [`ReadSource`]).
///
/// # Arguments
/// * `reader` The source reader.
///
/// # Generic arguments
/// * `T` Type of expected return value.
/// * `R` The type of source reader, in particular it can be a [`str`] or a [`Path`][std::path::Path].
///
/// Returns either a value of type T or a common error for deserialisation from both IEML input to IEML data
/// structure and from IEML data structure to Rust data structure.
///
/// *Note*:
/// The source reader can access the document system.
/// If you need to borrow data (e.g. deserialise `&str`), use [`parse_with_reader`], then
/// [`Data::view`][crate::data::data::Data::view] and [`Deserialize::deserialize`][de::Deserialize::deserialize].
///
/// # Example
///
/// ```rust
/// use serde_ieml::from_source;
///
/// let value = from_source::<String, _>("> hello");
///
/// assert_eq!(value, Ok("hello".into()));
/// ```
pub fn from_source<T, R>(reader: &R) -> Result<T>
where
    T: for<'data> de::Deserialize<'data>,
    R: ReadSource + ?Sized,
{
    from_source_advanced(reader, |token| Ok(token), ())
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use serde::Deserialize;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    enum Status {
        Married,
        Single,
    }

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    struct Human {
        name: String,
        age: u8,
        date: (u8, u8, u32),
        status: Status,
    }

    #[test]
    fn test_from_source() {
        let input = indoc! {"
            name: > John Doe
            age: 34
            date: [25, 7, 1990]
            status: Married
        "};
        assert_eq!(
            from_source::<Human, _>(input),
            Ok(Human {
                name: "John Doe".into(),
                age: 34,
                date: (25, 7, 1990),
                status: Status::Married,
            })
        );
    }
}
