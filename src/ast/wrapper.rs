use std::fmt::Write;

use super::{DError, Deserialize, Expr, Pair, Printable, Rule, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum Wrapper {
    Parentheses(Box<Expr>),
    Abs(Box<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Slice(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Serialize for Wrapper {
    fn serialize(&self) -> Printable {
        let tokens = match self {
            Self::Parentheses(x) => vec![
                "(".to_string().into(),
                x.serialize(),
                ")".to_string().into(),
            ],
            Self::Abs(x) => vec![
                "|".to_string().into(),
                x.serialize(),
                "|".to_string().into(),
            ],
            Self::Index(x, y) => vec![
                x.serialize(),
                "[".to_string().into(),
                y.serialize(),
                "]".to_string().into(),
            ],
            Self::Slice(x, y, z) => vec![
                x.serialize(),
                "[".to_string().into(),
                y.serialize(),
                ":".to_string().into(),
                z.serialize(),
                "]".to_string().into(),
            ],
        };
        tokens.into()
    }
}

impl Deserialize for Wrapper {
    fn deserialize(pair: Pair) -> Result<Self, DError> {
        use Rule as R;
        dbg!(pair.as_rule(), pair.as_str());
        match pair.as_rule() {
            R::parentheses => {
                let inner = pair.into_inner().next().unwrap();
                let inner = Expr::deserialize(inner)?;
                Ok(Self::Parentheses(inner.into()))
            }
            R::abs => {
                let inner = pair.into_inner().next().unwrap();
                let inner = Expr::deserialize(inner)?;
                Ok(Self::Abs(inner.into()))
            }
            R::index => {
                let mut pairs = pair.into_inner();
                let lhs = pairs.next().unwrap();
                let lhs = Expr::deserialize(lhs)?;
                let rhs = pairs.next().unwrap();
                let rhs = Expr::deserialize(rhs)?;
                Ok(Self::Index(lhs.into(), rhs.into()))
            }
            R::slice => {
                let mut pairs = pair.into_inner();
                let lhs = pairs.next().unwrap();
                let lhs = Expr::deserialize(lhs)?;
                let start = pairs.next().unwrap();
                let start = Expr::deserialize(start)?;
                let end = pairs.next().unwrap();
                let end = Expr::deserialize(end)?;
                Ok(Self::Slice(lhs.into(), start.into(), end.into()))
            }
            _ => Err(DError::UnknownRule),
        }
    }
}

#[cfg(test)]
macro_rules! parse {
    ($value:expr) => {{
        use crate::{Parser, PlanggyParser};
        Expr::deserialize(
            PlanggyParser::parse(crate::Rule::expr, $value)
                .unwrap()
                .next()
                .unwrap(),
        )
    }};
}

#[cfg(test)]
mod parentheses {
    use super::*;

    #[test]
    fn single() {
        assert_eq!(
            parse!("(789)"),
            Ok(Expr::Wrapped(Wrapper::Parentheses(789.into())))
        )
    }

    #[test]
    fn chain_rule() {
        let inner = Expr::Wrapped(Wrapper::Parentheses('c'.into()));
        assert_eq!(
            parse!("(('c'))"),
            Ok(Expr::Wrapped(Wrapper::Parentheses(inner.into())))
        )
    }
}

#[cfg(test)]
mod mixed {
    use super::*;

    #[test]
    fn chain_rule() {
        let inner = Expr::Wrapped(Wrapper::Abs("foo".to_string().into()));
        assert_eq!(
            parse!("(|\"foo\"|)"),
            Ok(Expr::Wrapped(Wrapper::Parentheses(inner.into())))
        )
    }
}
