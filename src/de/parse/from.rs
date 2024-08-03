use super::{cursor::Cursor, parse_complete::parse_complete, read_source::ReadSource};
use crate::{
    data::{data::Data, make},
    de::parse::{Error, ErrorKind, RateError},
};
use std::path::Path;

pub type AnchorsResult<'maker> =
    Result<make::MapToken<'maker>, RateError<'maker, ErrorKind, make::MapToken<'maker>>>;

/// Creates Data using a source reader (implements [`ReadSource`]).
/// Accepts a closure that generates external anchors that will be available in the document.
///
/// # Arguments
/// * `reader` The source reader.
/// * `anchors` The closure that generates external anchors.
///
/// *Note*: The source reader can access the document system.
///
/// # Example
///
/// ```rust
/// use serde_ieml::from_source_with_anchors;
/// use serde_ieml::data::name::Name;
/// use serde_ieml::data::make;
///
/// let data = from_source_with_anchors("@anchor", |token| {
///     let (token, _) = token.add(
///         Default::default(),
///         Name::new("anchor").unwrap(),
///         make::null(Default::default(), ()),
///     )?;
///     Ok(token)
/// }).unwrap();
/// 
/// let anchor_view = data.view().anchor().unwrap();
/// assert_eq!(anchor_view.name().as_str(), "anchor");
/// assert!(anchor_view.view().is_null());
/// ```
pub fn from_source_with_anchors<R: ReadSource + ?Sized, A>(
    reader: &R,
    anchors: A,
) -> Result<Data, Error>
where
    A: FnOnce(make::MapToken) -> AnchorsResult,
{
    make::make_document(
        Default::default(),
        reader.path().to_path_buf(),
        |token| Ok((anchors(token)?, Cursor::default())),
        |token| {
            let result = reader.read_source(token, |token, inner_cursor| {
                let (token, _) = parse_complete(reader.path(), inner_cursor)(token)?;
                Ok((token, Default::default()))
            });

            match result {
                Ok(i) => i,
                Err(token) => {
                    let error_kind = ErrorKind::NonexistentDocument;
                    let error = Error::new_with(Default::default(), Path::new(""), error_kind);
                    Err(RateError::Unrecoverable((token.error(), error)))
                }
            }
        },
    )
    .map(|(i, _)| i)
}

/// Creates Data using a source reader (implements [`ReadSource`]).
///
/// # Arguments
/// * `reader` The source reader.
///
/// *Note*: The source reader can access the document system. A more generic version of this function is [`from_source_with_anchors`].
///
/// # Example
///
/// ```rust
/// use serde_ieml::from_source;
/// use serde_ieml::data::make;
///
/// let data = from_source("> hello").unwrap();
/// 
/// assert_eq!(data.view().string().unwrap().string(), "hello");
/// ```
pub fn from_source<R: ReadSource + ?Sized>(reader: &R) -> Result<Data, Error> {
    from_source_with_anchors(reader, |token| Ok(token))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_source() {
        let begin_mark = Default::default();

        let data = from_source("> hello").unwrap();

        let result_f = make::string::<_, ErrorKind, _>(begin_mark, (), "hello");
        let (result, _) = make::make(begin_mark, result_f).unwrap();

        assert_eq!(data, result);
    }
}
