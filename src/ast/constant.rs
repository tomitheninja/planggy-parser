use super::{DError, Deserialize, Pair, Printable, Rule, Serialize};
use derive_more::{Display, From, TryInto};

pub type Int = i32;
pub type Float = f64;

#[derive(Debug, Clone, PartialEq, From, TryInto, Display)]
pub enum Constant {
    #[display(fmt = "{}", _0)]
    Int(Int),
    Float(Float),
    #[display(fmt = "'{}'", _0)]
    Char(char),
    #[display(fmt = "\"{}\"", _0)]
    String(String),
    Boolean(bool),
}

impl Serialize for Constant {
    fn serialize(&self) -> Printable {
        use Constant::*;
        let val = match self {
            Float(x) => {
                if x.floor() == *x {
                    format!("{}.0", x)
                } else {
                    format!("{}", x)
                }
            }
            Boolean(true) => "IGAZ".to_string(),
            Boolean(false) => "HAMIS".to_string(),
            _ => format!("{}", self),
        };
        val.into()
    }
}

impl Deserialize for Constant {
    fn deserialize(pair: Pair) -> Result<Self, DError> {
        use Rule::{
            boolean, boolean_false, boolean_true, character, constant, float, int, number, string,
        };
        dbg!(pair.as_rule(), pair.as_str());
        match pair.as_rule() {
            // bool
            boolean_true => Ok(true.into()),
            boolean_false => Ok(false.into()),
            // int
            int => Ok(Self::Int(
                pair.as_str()
                    .parse()
                    .map_err(|_| DError::ConstValueTooLarge)?,
            )),
            // float
            float => Ok(Self::Float(
                pair.as_str()
                    .parse()
                    .map_err(|_| DError::ConstValueTooLarge)?,
            )),
            // char
            character => Ok(({
                let mut s = pair.as_str().chars();
                match s.next().unwrap() {
                    '\\' => s.next().unwrap(),
                    c => c,
                }
            })
            .into()),
            // string
            string => Ok(pair.as_str().to_string().into()),
            // wrappers
            boolean | constant | number => Self::deserialize(pair.into_inner().next().unwrap()),
            _ => Err(DError::UnknownRule),
        }
    }
}

#[cfg(test)]
macro_rules! parse {
    ($value:expr) => {{
        use crate::{Parser, PlanggyParser};
        Constant::deserialize(
            PlanggyParser::parse(crate::Rule::constant, $value)
                .unwrap()
                .next()
                .unwrap(),
        )
    }};
}

#[cfg(test)]
mod boolean {
    use super::*;

    #[test]
    fn uppercase_igaz() {
        assert_eq!(parse!("IGAZ"), Ok(true.into()))
    }

    #[test]
    fn uppercase_hamis() {
        assert_eq!(parse!("HAMIS"), Ok(false.into()))
    }
}

#[cfg(test)]
mod number {
    use super::*;

    #[test]
    fn int_one() {
        assert_eq!(parse!("1"), Ok(1.into()))
    }

    #[test]
    fn int_minus_one() {
        assert_eq!(parse!("-1"), Ok((-1).into()))
    }

    #[test]
    fn int_multi_digit() {
        assert_eq!(parse!("+123"), Ok(123.into()))
    }
    #[test]
    fn too_large() {
        assert_eq!(parse!("99999999999999999"), Err(DError::ConstValueTooLarge))
    }

    #[test]
    fn float_minus_one() {
        assert_eq!(parse!("-1.0"), Ok((-1.0).into()))
    }
}

#[cfg(test)]
mod character {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(parse!("'a'"), Ok('a'.into()))
    }

    #[test]
    fn uppercase_a() {
        assert_eq!(parse!("'A'"), Ok('A'.into()))
    }

    #[test]
    fn digit() {
        assert_eq!(parse!("'7'"), Ok('7'.into()))
    }

    #[test]
    fn space() {
        assert_eq!(parse!("' '"), Ok(' '.into()))
    }

    #[test]
    fn newline() {
        assert_eq!(parse!("'\n'"), Ok('\n'.into()))
    }

    #[test]
    fn single_quote() {
        assert_eq!(parse!(r"'\''"), Ok('\''.into()))
    }
}

#[cfg(test)]
mod string {
    use super::*;

    #[test]
    fn words() {
        assert_eq!(
            parse!(r#""lorem ipsum 123""#),
            Ok("lorem ipsum 123".to_string().into())
        )
    }

    #[test]
    fn special_chars() {
        assert_eq!(
            parse!(r#""lorem\t\nipsum 123""#),
            Ok("lorem\\t\\nipsum 123".to_string().into())
        )
    }

    #[test]
    fn double_quote() {
        assert_eq!(
            parse!("\"lorem\\\"ipsum\""),
            Ok("lorem\\\"ipsum".to_string().into())
        )
    }
}
