use nom::error::{ErrorKind, ParseError};
use nom::InputLength;
use nom::{Err, IResult, Parser};

/// Applies the parser `f` until the parser `g` produces a result, counting the results.
///
/// Returns a count of the results of `f` and the result of `g`.
///
/// `f` keeps going so long as `g` produces [`Err::Error`]. To instead chain an error up, see [`cut`][crate::combinator::cut].
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use serde_ieml::parse::utils::nom::many_till_count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, (usize, &str)> {
///   many_till_count(tag("abc"), tag("end"))(s)
/// };
///
/// assert_eq!(parser("abcabcend"), Ok(("", (2, "end"))));
/// assert_eq!(parser("abc123end"), Err(Err::Error(Error::new("123end", ErrorKind::Tag))));
/// assert_eq!(parser("123123end"), Err(Err::Error(Error::new("123123end", ErrorKind::Tag))));
/// assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
/// assert_eq!(parser("abcendefg"), Ok(("efg", (1, "end"))));
/// ```
pub fn many_till_count<I, O, P, E, F, G>(
    mut f: F,
    mut g: G,
) -> impl FnMut(I) -> IResult<I, (usize, P), E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, P, E>,
    E: ParseError<I>,
{
    move |mut i: I| {
        let mut count = 0;
        loop {
            let len = i.input_len();
            match g.parse(i.clone()) {
                Ok((i1, o)) => return Ok((i1, (count, o))),

                Err(Err::Error(_)) => match f.parse(i.clone()) {
                    Err(Err::Error(err)) => {
                        return Err(Err::Error(E::append(i, ErrorKind::ManyTill, err)))
                    }

                    Err(e) => return Err(e),

                    Ok((i1, _)) => {
                        // infinite loop check: the parser must always consume
                        if i1.input_len() == len {
                            return Err(Err::Error(E::from_error_kind(i1, ErrorKind::ManyTill)));
                        }

                        count += 1;
                        i = i1;
                    }
                },

                Err(e) => return Err(e),
            }
        }
    }
}
