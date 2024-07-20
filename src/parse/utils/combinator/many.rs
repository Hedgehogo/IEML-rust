use nom::error::{ErrorKind, ParseError};
use nom::InputLength;
use nom::{Err, IResult, Parser};

/// Repeats the embedded parser `m..=n` times
///
/// This stops before `n` when the parser returns [`Err::Error`] and returns a count of the results. To instead chain an error up, see
/// [`cut`][nom::combinator::cut].
///
/// # Arguments
/// * `m` The minimum number of iterations.
/// * `n` The maximum number of iterations.
/// * `f` The parser to apply.
///
/// *Note*: If the parser passed to `many1` accepts empty inputs
/// (like `alpha0` or `digit0`), `many1` will return an error,
/// to prevent going into an infinite loop.
///
/// ```rust
/// # use nom::{Err, error::ErrorKind, Needed, IResult};
/// use serde_ieml::parse::utils::combinator::many::many_m_n_count;
/// use nom::bytes::complete::tag;
///
/// fn parser(s: &str) -> IResult<&str, usize> {
///   many_m_n_count(0, 2, tag("abc"))(s)
/// }
///
/// assert_eq!(parser("abcabc"), Ok(("", 2)));
/// assert_eq!(parser("abc123"), Ok(("123", 1)));
/// assert_eq!(parser("123123"), Ok(("123123", 0)));
/// assert_eq!(parser(""), Ok(("", 0)));
/// assert_eq!(parser("abcabcabc"), Ok(("abc", 2)));
/// ```
pub fn many_m_n_count<I, O, E, F>(
    min: usize,
    max: usize,
    mut parse: F,
) -> impl FnMut(I) -> IResult<I, usize, E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    move |mut input: I| {
        if min > max {
            return Err(Err::Failure(E::from_error_kind(input, ErrorKind::ManyMN)));
        }

        for count in 0..max {
            let len = input.input_len();
            match parse.parse(input.clone()) {
                Ok((tail, _)) => {
                    // infinite loop check: the parser must always consume
                    if tail.input_len() == len {
                        return Err(Err::Error(E::from_error_kind(input, ErrorKind::ManyMN)));
                    }

                    input = tail;
                }
                
                Err(Err::Error(e)) => {
                    if count < min {
                        return Err(Err::Error(E::append(input, ErrorKind::ManyMN, e)));
                    } else {
                        return Ok((input, count));
                    }
                }

                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok((input, max))
    }
}

/// Applies the parser `f` until the parser `g` produces a result, counting the results.
///
/// Returns a count of the results of `f` and the result of `g`.
///
/// `f` keeps going so long as `g` produces [`Err::Error`]. To instead chain an error up, see [`cut`][nom::combinator::cut].
///
/// ```rust
/// # use nom::{Err, error::{Error, ErrorKind}, Needed, IResult};
/// use serde_ieml::parse::utils::combinator::many::many_till_count;
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
