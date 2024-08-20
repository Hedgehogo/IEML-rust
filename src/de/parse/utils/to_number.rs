use super::{combinator::parse::skip_blank_line, number::Number};

pub trait ToNumber: Number {
    fn parse_exponent(number: Self, exponent: isize, radix: u8) -> Option<Self>;

    fn parse_fractional_part(input: &str, radix: u8, number: Self) -> Option<(&str, Self)>;

    fn parse_minus(input: &str) -> (&str, bool);

    fn add_minus(number: Self, minus: bool) -> Self;
}

macro_rules! impl_parse_number {
    ($parse_exponent:ident, $parse_fractional_part:ident, $parse_minus:ident, $add_minus:ident, ($($T:ty),*)) => {
        $(
            impl ToNumber for $T {
                fn parse_exponent(number: Self, exponent: isize, radix: u8) -> Option<Self> {
                    $parse_exponent(number, exponent, radix)
                }

                fn parse_fractional_part(input: &str, radix: u8, number: Self) -> Option<(&str, Self)> {
                    $parse_fractional_part(input, radix, number)
                }

                fn parse_minus(input: &str) -> (&str, bool) {
                    $parse_minus(input)
                }

                fn add_minus(number: Self, minus: bool) -> Self {
                    $add_minus(number, minus)
                }
            }
        )*
    };
}

pub fn add_minus_signed<T: Number + std::ops::Neg<Output = T>>(number: T, minus: bool) -> T {
    if minus {
        -number
    } else {
        number
    }
}

pub fn add_minus_unsigned<T: Number>(number: T, _minus: bool) -> T {
    number
}

pub fn parse_minus_signed(input: &str) -> (&str, bool) {
    let mut chars = input.chars();

    match chars.next() {
        Some(i) => match i {
            '-' => (chars.as_str(), true),
            '+' => (chars.as_str(), true),
            _ => (input, false),
        },
        None => (input, false),
    }
}

pub fn parse_minus_unsigned(input: &str) -> (&str, bool) {
    (input, false)
}

pub fn to_digit(input: char, radix: u8) -> Option<u8> {
    match input {
        '0'..='9' => Some((input as u8) - b'0'),
        'A'..='Z' => Some(10 + (input as u8) - b'A'),
        _ => None,
    }
    .and_then(|i| if i < radix { Some(i) } else { None })
}

pub fn parse_number_part<T: Number>(input: &str, radix: u8) -> Option<(&str, (T, T))> {
    let mut chars = input.chars();
    let mut new_input;
    let mut value = T::zero();
    let mut factor = T::one();
    loop {
        new_input = chars.as_str();
        if let Some(i) = chars.next() {
            if i == '_' {
                continue;
            }
            match to_digit(i, radix) {
                Some(digit) => {
                    if T::max_value() / factor.clone() >= T::from_u8(radix)? {
                        factor *= T::from_u8(radix)?;
                        value *= T::from_u8(radix)?;
                        if T::max_value() - value.clone() >= T::from_u8(digit)? {
                            value += T::from_u8(digit)?;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                None => break,
            }
        } else {
            break;
        }
    }
    Some((new_input, (value, factor)))
}

pub fn parse_fractional_part_integer<T: Number>(
    input: &str,
    _radix: u8,
    number: T,
) -> Option<(&str, T)> {
    Some((input, number))
}

pub fn parse_fractional_part_float<T: Number>(
    input: &str,
    radix: u8,
    mut number: T,
) -> Option<(&str, T)> {
    let mut chars = input.chars();
    match chars.next() {
        Some('.') => {
            let (new_input, (fractional_part, factor)) =
                parse_number_part::<T>(chars.as_str(), radix)?;
            (factor > T::one()).then(|| {
                number += fractional_part / factor;
                (new_input, number)
            })
        }
        _ => Some((input, number)),
    }
}

pub fn parse_number<T: ToNumber>(input: &str, radix: u8) -> Option<(&str, T)> {
    let (new_input, (number, factor)) = parse_number_part(input, radix)?;
    if factor > T::from_u8(1)? {
        T::parse_fractional_part(new_input, radix, number)
    } else {
        None
    }
}

pub fn parse_number_radix<T: ToNumber>(input: &str) -> Option<(&str, (T, u8))> {
    let (new_input, minus) = T::parse_minus(input);
    let (new_input, (number_or_radix, factor)) = parse_number_part::<T>(new_input, 10)?;
    if factor > T::one() {
        let mut chars = new_input.chars();
        match chars.next() {
            Some('\'') => {
                if number_or_radix >= T::one() && number_or_radix <= T::from_u8(36)? {
                    let radix = number_or_radix.to_u8()?;
                    let (new_input, number) = parse_number(chars.as_str(), radix)?;
                    Some((new_input, (T::add_minus(number, minus), radix)))
                } else {
                    None
                }
            }
            _ => {
                let (new_input, number) = T::parse_fractional_part(new_input, 10, number_or_radix)?;
                Some((new_input, (T::add_minus(number, minus), 10)))
            }
        }
    } else {
        None
    }
}

pub fn parse_exponent_integer<T: Number + num_traits::CheckedMul>(
    number: T,
    exponent: isize,
    radix: u8,
) -> Option<T> {
    if exponent > 0 {
        let factor = num_traits::checked_pow(T::from_u8(radix)?, exponent as usize)?;
        (T::max_value() / factor.clone() >= number).then(|| number * factor)
    } else {
        let factor = num_traits::checked_pow(T::from_u8(radix)?, -exponent as usize)?;
        Some(number / factor)
    }
}

pub fn parse_exponent_float<T: Number>(number: T, exponent: isize, radix: u8) -> Option<T> {
    if exponent > 0 {
        let factor = num_traits::pow(T::from_u8(radix)?, exponent as usize);
        Some(number * factor)
    } else {
        let factor = num_traits::pow(T::from_u8(radix)?, -exponent as usize);
        Some(number / factor)
    }
}

pub fn parse_number_scientific<T: ToNumber>(input: &str) -> Option<(&str, T)> {
    let (new_input, (number, radix)) = parse_number_radix(input)?;
    let mut chars = new_input.chars();
    match chars.next() {
        Some('e') => {
            let new_input = chars.as_str();
            match chars.next() {
                Some('0'..='9' | '-') => {
                    let (new_input, (exponent, _)) = parse_number_radix::<isize>(new_input)?;
                    let number = T::parse_exponent(number, exponent, radix)?;
                    return Some((new_input, number));
                }
                _ => {}
            }
        }
        _ => {}
    }
    Some((new_input, number))
}

impl_parse_number!(
    parse_exponent_float,
    parse_fractional_part_float,
    parse_minus_signed,
    add_minus_signed,
    (f32, f64)
);
impl_parse_number!(
    parse_exponent_integer,
    parse_fractional_part_integer,
    parse_minus_signed,
    add_minus_signed,
    (i8, i16, i32, i64, i128, isize)
);
impl_parse_number!(
    parse_exponent_integer,
    parse_fractional_part_integer,
    parse_minus_unsigned,
    add_minus_unsigned,
    (u8, u16, u32, u64, u128, usize)
);

pub fn to_number<T: ToNumber>(input: &str) -> Option<T> {
    let (input, number) = parse_number_scientific(input)?;
    let cursor = skip_blank_line((input, Default::default()).into());
    cursor.input.is_empty().then_some(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digit() {
        assert_eq!(to_digit('0', 10), Some(0));
        assert_eq!(to_digit('9', 10), Some(9));
        assert_eq!(to_digit('A', 10), None);

        assert_eq!(to_digit('0', 16), Some(0));
        assert_eq!(to_digit('9', 16), Some(9));
        assert_eq!(to_digit('A', 16), Some(10));
        assert_eq!(to_digit('F', 16), Some(15));
        assert_eq!(to_digit('G', 16), None);
    }

    #[test]
    fn test_parse_number_part() {
        assert_eq!(
            parse_number_part::<i32>("1_200", 10).unwrap().1,
            (1_200, 10_000)
        );
        assert_eq!(parse_number_part::<i32>("F8", 16).unwrap().1, (0xF8, 0x100));
        assert_eq!(parse_number_part::<i32>("A5", 10).unwrap().1, (0, 1));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number::<i32>("120", 10).unwrap().1, 120);
        assert_eq!(parse_number::<i32>("120.5", 10).unwrap().1, 120);
        assert_eq!(parse_number::<f32>("120.5", 10).unwrap().1, 120.5);
        assert_eq!(parse_number::<i32>("F8", 16).unwrap().1, 0xF8);
        assert_eq!(parse_number::<i32>("A5", 10), None);
    }

    #[test]
    fn test_parse_number_radix() {
        assert_eq!(parse_number_radix::<i32>("120").unwrap().1, (120, 10));
        assert_eq!(parse_number_radix::<f32>("120.5").unwrap().1, (120.5, 10));
        assert_eq!(parse_number_radix::<i32>("16'F8").unwrap().1, (0xF8, 16));
        assert_eq!(parse_number_radix::<f32>("-2'101.1").unwrap().1, (-5.5, 2));
    }

    #[test]
    fn test_parse_number_scientific() {
        assert_eq!(parse_number_scientific::<i32>("120"), Some(("", 120)));
        assert_eq!(parse_number_scientific::<f32>("2'101.1"), Some(("", 5.5)));
        assert_eq!(
            parse_number_scientific::<i32>("120e2'10"),
            Some(("", 12_000))
        );
        assert_eq!(
            parse_number_scientific::<f32>("2'101.1e-3"),
            Some(("", 0.6875))
        );
        assert_eq!(
            parse_number_scientific::<f32>("2'101.1edit"),
            Some(("edit", 5.5))
        );
    }

    #[test]
    fn test_to_number() {
        assert_eq!(to_number::<i32>("2'10e1"), Some(4));
        assert_eq!(to_number::<i32>("2'10e1  \t "), Some(4));
        assert_eq!(to_number::<i32>("2'10e1 # hello"), Some(4));
        assert_eq!(to_number::<i32>("2'10e1k"), None);
    }
}
