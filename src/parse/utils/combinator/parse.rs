use super::super::super::cursor::Cursor;
use super::cursor::*;
use super::many::*;
use nom::{
    combinator::{not, opt, peek},
    multi::*,
    sequence::tuple,
    *,
};

pub fn skip_line_ending(input: Cursor) -> IResult<Cursor, ()> {
    tuple((opt(char('\r')), char('\n')))
        .map(|_| ())
        .parse(input)
}

pub fn skip_not_line_ending(input: Cursor) -> IResult<Cursor, ()> {
    tuple((not(skip_line_ending), anychar))
        .map(|_| ())
        .parse(input)
}

pub fn skip_indent(indent: usize) -> impl FnMut(Cursor) -> IResult<Cursor, ()> {
    move |input| {
        let (input, _) = many_m_n_count(indent, indent, char('\t'))(input)?;
        Ok((input, ()))
    }
}

pub fn skip_space(input: Cursor) -> IResult<Cursor, ()> {
    char(' ')
        .map(|_| ())
        .or(peek(skip_line_ending))
        .parse(input)
}

pub fn skip_blank_line(input: Cursor) -> Cursor {
    let (input, _) = many0_count(one_of(" \t"))(input)
        .expect("Internal error in `skip_blank_line` function operation.");
    let mut parse_comment = tuple((char('#'), one_of("! "), many0_count(skip_not_line_ending)));
    parse_comment(input)
        .map(|(input, _)| input)
        .unwrap_or(input)
}

pub fn skip_blank_lines_ln(input: Cursor) -> IResult<Cursor, usize> {
    many1_count(|input| {
        let input = skip_blank_line(input);
        skip_line_ending(input)
    })(input)
}

pub fn match_line<'input>(input: Cursor<'input>) -> (Cursor<'input>, &'input str) {
    let (output, result) = recognize(many0_count(skip_not_line_ending))(input)
        .expect("Internal error in `match_line` function operation.");
    (output, result.input)
}

pub fn match_name<'input>(input: Cursor<'input>) -> IResult<Cursor<'input>, &str> {
    let match_special = recognize(tuple((char(':'), skip_space)));
    let (output, (_, special)) = many_till_count(skip_not_line_ending, match_special)(input)?;

    let bytes = input.input.len() - output.input.len() - special.input.len();
    let (result, _) = input.input.split_at(bytes);

    Ok((output, result))
}

#[cfg(test)]
mod tests {
    use crate::data::mark::Mark;

    use super::*;

    #[test]
    fn test_skip_newline() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_line_ending(("\nhello", mark).into()),
            Ok((("hello", Mark::new(16, 0)).into(), ()))
        );
        assert!(skip_line_ending(("hello", mark).into()).is_err());
    }

    #[test]
    fn test_skip_indent() {
        let input = "\t\t\thello";
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_indent(0)((input, mark).into()),
            Ok((("\t\t\thello", Mark::new(15, 10)).into(), ()))
        );
        assert_eq!(
            skip_indent(2)((input, mark).into()),
            Ok((("\thello", Mark::new(15, 12)).into(), ()))
        );
        assert_eq!(
            skip_indent(3)((input, mark).into()),
            Ok((("hello", Mark::new(15, 13)).into(), ()))
        );
        assert!(skip_indent(4)((input, mark).into()).is_err());
    }

    #[test]
    fn test_skip_blank_line() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_blank_line(("\t  hello", mark).into()),
            ("hello", Mark::new(15, 13)).into()
        );
        assert_eq!(
            skip_blank_line(("\t # fg\n b", mark).into()),
            ("\n b", Mark::new(15, 16)).into()
        );
        assert_eq!(
            skip_blank_line(("\t#sadh ", mark).into()),
            ("#sadh ", Mark::new(15, 11)).into()
        );
        assert_eq!(
            skip_blank_line(("#sadh ", mark).into()),
            ("#sadh ", Mark::new(15, 10)).into()
        );
    }

    #[test]
    fn test_skip_blank_lines_ln() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            skip_blank_lines_ln((" # hello\n\t", mark).into()),
            Ok((("\t", Mark::new(16, 0)).into(), 1))
        );
        assert_eq!(
            skip_blank_lines_ln((" # hello\n\t \t \n world", mark).into()),
            Ok(((" world", Mark::new(17, 0)).into(), 2))
        );
        assert_eq!(
            skip_blank_lines_ln((" # hello\nhello", mark).into()),
            Ok((("hello", Mark::new(16, 0)).into(), 1))
        );
        assert!(skip_blank_lines_ln((" #hello\nhello", mark).into()).is_err());
    }

    #[test]
    fn test_match_name() {
        let mark = Mark::new(15, 10);
        assert_eq!(
            match_name(("key: value", mark).into()),
            Ok((("value", Mark::new(15, 15)).into(), "key"))
        );
        assert_eq!(
            match_name(("key:\n", mark).into()),
            Ok((("\n", Mark::new(15, 14)).into(), "key"))
        );
        assert_eq!(
            match_name(("key key: ", mark).into()),
            Ok((("", Mark::new(15, 19)).into(), "key key"))
        );
        assert!(skip_blank_lines_ln(("key:", mark).into()).is_err());
        assert!(skip_blank_lines_ln(("key\n: ", mark).into()).is_err());
    }
}
