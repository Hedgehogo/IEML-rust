use super::super::super::cursor::Cursor;
use nom::{
    error::{Error, ErrorKind, ParseError},
    IResult, Parser,
};

/// Matches one code point as a character.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, IResult};
/// # use serde_ieml::{data::mark::Mark, de::parse::cursor::Cursor};
/// use serde_ieml::de::parse::utils::combinator::cursor::anychar;
///
/// fn parser(input: Cursor) -> IResult<Cursor, char> {
///     anychar(input)
/// }
///
/// let mark = Default::default();
/// assert_eq!(parser(("abc", mark).into()), Ok((("bc", Mark::new(0, 1)).into(),'a')));
/// assert_eq!(parser(("", mark).into()), Err(Err::Error(Error::new(("", mark).into(), ErrorKind::Eof))));
/// ```
pub fn anychar(input: Cursor) -> IResult<Cursor, char> {
    let mut iter = input.chars();
    match iter.next() {
        Some(ch) => Ok((iter.cursor(), ch)),
        None => Err(nom::Err::Error(Error::from_error_kind(
            input,
            ErrorKind::Eof,
        ))),
    }
}

/// Recognizes one character.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, IResult};
/// # use serde_ieml::{data::mark::Mark, de::parse::cursor::Cursor};
/// use serde_ieml::de::parse::utils::combinator::cursor::char;
///
/// fn parser(input: Cursor) -> IResult<Cursor, char> {
///     char('a')(input)
/// }
///
/// let mark = Default::default();
/// assert_eq!(parser(("abc", mark).into()), Ok((("bc", Mark::new(0, 1)).into(), 'a')));
/// assert_eq!(parser((" abc", mark).into()), Err(Err::Error(Error::new((" abc", mark).into(), ErrorKind::Char))));
/// assert_eq!(parser(("bc", mark).into()), Err(Err::Error(Error::new(("bc", mark).into(), ErrorKind::Char))));
/// assert_eq!(parser(("", mark).into()), Err(Err::Error(Error::new(("", mark).into(), ErrorKind::Char))));
/// ```
pub fn char(ch: char) -> impl FnMut(Cursor) -> IResult<Cursor, char> {
    move |input| {
        let mut iter = input.chars();
        match iter.next().and_then(|i| (i == ch).then_some(())) {
            Some(_) => Ok((iter.cursor(), ch)),
            None => Err(nom::Err::Error(Error::from_char(input, ch))),
        }
    }
}

/// Recognizes one of the provided characters.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind, ParseError}};
/// # use serde_ieml::{data::mark::Mark, de::parse::cursor::Cursor};
/// use serde_ieml::de::parse::utils::combinator::cursor::one_of;
///
/// let mark = Default::default();
/// assert_eq!(one_of("abc")(("b", mark).into()), Ok((("", Mark::new(0, 1)).into(), 'b')));
/// assert_eq!(one_of("a")(("bc", mark).into()), Err(Err::Error(Error::from_error_kind(("bc", mark).into(), ErrorKind::OneOf))));
/// assert_eq!(one_of("a")(("", mark).into()), Err(Err::Error(Error::from_error_kind(("", mark).into(), ErrorKind::OneOf))));
/// ```
pub fn one_of<'input>(
    list: &'input str,
) -> impl FnMut(Cursor<'input>) -> IResult<Cursor<'input>, char> {
    move |input| {
        let mut iter = input.chars();
        match iter.next().and_then(|i| list.contains(i).then_some(i)) {
            Some(i) => Ok((iter.cursor(), i)),
            None => Err(nom::Err::Error(Error::from_error_kind(
                input,
                ErrorKind::OneOf,
            ))),
        }
    }
}

/// Recognizes a character that is not in the provided characters.
///
/// *Complete version*: Will return an error if there's not enough input data.
/// # Example
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind, ParseError}};
/// # use serde_ieml::{data::mark::Mark, de::parse::cursor::Cursor};
/// use serde_ieml::de::parse::utils::combinator::cursor::none_of;
///
/// let mark = Default::default();
/// assert_eq!(none_of("abc")(("z", mark).into()), Ok((("", Mark::new(0, 1)).into(), 'z')));
/// assert_eq!(none_of("ab")(("a", mark).into()), Err(Err::Error(Error::from_error_kind(("a", mark).into(), ErrorKind::NoneOf))));
/// assert_eq!(none_of("a")(("", mark).into()), Err(Err::Error(Error::from_error_kind(("", mark).into(), ErrorKind::NoneOf))));
/// ```
pub fn none_of<'input>(
    chars: &'input str,
) -> impl FnMut(Cursor<'input>) -> IResult<Cursor<'input>, char> {
    move |input| {
        let mut iter = input.chars();
        match iter.next().and_then(|i| (!chars.contains(i)).then_some(i)) {
            Some(i) => Ok((iter.cursor(), i)),
            None => Err(nom::Err::Error(Error::from_error_kind(
                input,
                ErrorKind::NoneOf,
            ))),
        }
    }
}

/// If the child parser was successful, return the consumed input as produced value.
///
/// # Example
///
/// ```rust
/// # use nom::{Err, Parser, error::{Error, ErrorKind, ParseError}};
/// # use serde_ieml::{data::mark::Mark, de::parse::cursor::Cursor};
/// use nom::combinator::value;
/// use serde_ieml::de::parse::utils::combinator::cursor::char;
/// use serde_ieml::de::parse::utils::combinator::cursor::recognize;
///
/// let mut parser = recognize(char('0').or(char('1')));
///
/// let mark = Default::default();
/// assert_eq!(parser(("1def", mark).into()), Ok((("def", Mark::new(0, 1)).into(), ("1", mark).into())));
/// assert_eq!(parser(("2def", mark).into()), Err(Err::Error(Error::from_error_kind(("2def", mark).into(), ErrorKind::Char))));
/// ```
pub fn recognize<'input, O, F>(
    mut f: F,
) -> impl FnMut(Cursor<'input>) -> IResult<Cursor<'input>, Cursor<'input>>
where
    F: Parser<Cursor<'input>, O, Error<Cursor<'input>>>,
{
    move |input| {
        let (output, _) = f.parse(input)?;
        let bytes = input.input.len() - output.input.len();
        let (result, _) = input.input.split_at(bytes);
        Ok((output, (result, input.mark).into()))
    }
}

/// if the child parser was successful, return the consumed input with the output
/// as a tuple. Functions similarly to [recognize](fn.recognize.html) except it
/// returns the parser output as well.
///
/// This can be useful especially in cases where the output is not the same type
/// as the input, or the input is a user defined type.
///
/// Returned tuple is of the format `(consumed input, produced output)`.
///
/// # Example
///
/// ```rust
/// # use nom::{Err, Parser, error::{Error, ErrorKind, ParseError}};
/// # use serde_ieml::{data::mark::Mark, de::parse::cursor::Cursor};
/// use nom::combinator::value;
/// use serde_ieml::de::parse::utils::combinator::cursor::char;
/// use serde_ieml::de::parse::utils::combinator::cursor::consumed;
///
/// let mut parser = consumed(char('0').or(char('1')));
///
/// let mark = Default::default();
/// assert_eq!(parser(("1def", mark).into()), Ok((("def", Mark::new(0, 1)).into(), (("1", mark).into(), '1'))));
/// assert_eq!(parser(("2def", mark).into()), Err(Err::Error(Error::from_error_kind(("2def", mark).into(), ErrorKind::Char))));
/// ```
pub fn consumed<'input, O, F>(
    mut f: F,
) -> impl FnMut(Cursor<'input>) -> IResult<Cursor<'input>, (Cursor<'input>, O)>
where
    F: Parser<Cursor<'input>, O, Error<Cursor<'input>>>,
{
    move |input| {
        let (output, produced_result) = f.parse(input)?;
        let bytes = input.input.len() - output.input.len();
        let (result, _) = input.input.split_at(bytes);
        Ok((output, ((result, input.mark).into(), produced_result)))
    }
}
