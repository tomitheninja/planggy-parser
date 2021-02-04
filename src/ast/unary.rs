use super::{DError, Deserialize, Expr, Pair, Printable, Rule, Serialize};
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Display)]
pub enum UnaryOpCode {
    #[display(fmt = "NEM ")]
    Not,
    #[display(fmt = "-")]
    Neg,
    #[display(fmt = "SIN ")]
    Sin,
    #[display(fmt = "COS ")]
    Cos,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr(pub UnaryOpCode, pub Box<Expr>);

impl Serialize for UnaryOpCode {
    fn serialize(&self) -> Printable {
        format!("{}", self).into()
    }
}

impl From<(UnaryOpCode, Expr)> for UnaryExpr {
    fn from(x: (UnaryOpCode, Expr)) -> Self {
        Self(x.0, Box::new(x.1))
    }
}

impl Serialize for UnaryExpr {
    fn serialize(&self) -> Printable {
        vec![self.0.serialize(), (*self.1).serialize()].into()
    }
}

impl Deserialize for UnaryOpCode {
    fn deserialize(pair: Pair) -> Result<Self, DError> {
        use Rule as R;
        dbg!(pair.as_rule(), pair.as_str());
        Ok(match pair.as_rule() {
            R::unary_op => Self::deserialize(pair.into_inner().next().unwrap())?,
            R::not => Self::Not,
            R::sin => Self::Sin,
            R::cos => Self::Cos,
            _ => Err(DError::UnknownRule)?,
        })
    }
}

impl Deserialize for UnaryExpr {
    fn deserialize(pair: Pair) -> Result<Self, DError> {
        dbg!(pair.as_rule(), pair.as_str());
        if pair.as_rule() == Rule::unary {
            let mut pairs = pair.into_inner();
            // Operation
            let op = pairs.next().unwrap();
            let op = UnaryOpCode::deserialize(op.clone())?;
            // Whitespace
            let mut next = pairs.next().unwrap();
            if next.as_rule() == Rule::WS {
                next = pairs.next().unwrap();
            }
            // Expression
            let expr = Expr::deserialize(next)?;
            // Whitespace
            while let Some(next) = pairs.next() {
                if next.as_rule() != Rule::WS {
                    return Err(DError::UnknownExpression);
                }
            }
            Ok(Self(op, expr.into()))
        } else {
            Err(DError::UnknownRule)
        }
    }
}

#[cfg(test)]
macro_rules! parse {
    ($value:expr) => {{
        use crate::{Parser, PlanggyParser};
        let parse_result = PlanggyParser::parse(crate::Rule::unary, $value);
        dbg!(&parse_result);
        UnaryExpr::deserialize(parse_result.unwrap().next().unwrap())
    }};
}

#[cfg(test)]
mod since_op {
    use super::*;

    #[test]
    fn not_true() {
        assert_eq!(
            parse!("NEM IGAZ"),
            Ok((UnaryOpCode::Not, true.into()).into())
        )
    }
}

#[cfg(test)]
mod chain_rule {
    use super::*;

    #[test]
    fn sin_cos_float() {
        let inner = Expr::from((UnaryOpCode::Cos, Expr::from(0.5)));
        assert_eq!(parse!("SIN COS 0.5"), Ok((UnaryOpCode::Sin, inner).into()));
    }
}

#[cfg(test)]
mod wrapper_syntax {
    use super::*;

    #[test]
    fn unary_parentheses() {
        assert!(parse!("SIN(0.5)").is_ok());
    }
}
