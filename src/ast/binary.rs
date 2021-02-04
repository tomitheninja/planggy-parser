use super::{Constant, DError, Deserialize, Expr, Pair, Printable, Rule, Serialize};
use derive_more::{Display, From, TryInto};
use pest::RuleType;

// keep in execution order
#[derive(Debug, PartialOrd, Clone, PartialEq, From, Display)]
pub enum BinaryOpCode {
    #[display(fmt = "^")]
    Pow,
    #[display(fmt = "@")]
    Search,
    #[display(fmt = "*")]
    Mul,
    #[display(fmt = "/")]
    Div,
    #[display(fmt = "+")]
    Add,
    #[display(fmt = "-")]
    Sub,
    #[display(fmt = "=")]
    Eq,
    #[display(fmt = "/=")]
    Ne,
}

impl BinaryOpCode {
    fn tiers() -> &'static [&'static [Self]] {
        use BinaryOpCode::*;
        &[&[Pow, Search], &[Mul, Div], &[Add, Sub], &[Eq, Ne]]
    }
}

impl Serialize for BinaryOpCode {
    fn serialize(&self) -> Printable {
        format!("{}", self).into()
    }
}

impl Deserialize for BinaryOpCode {
    fn deserialize(pair: Pair) -> Result<Self, DError> {
        use Rule as R;
        dbg!(pair.as_rule(), pair.as_str());
        Ok(match pair.as_rule() {
            R::pow => Self::Pow,
            R::search => Self::Search,
            R::mul => Self::Mul,
            R::div => Self::Div,
            R::add => Self::Add,
            R::sub => Self::Sub,
            R::eq => Self::Eq,
            R::ne => Self::Ne,
            R::binary_op => Self::deserialize(pair.into_inner().next().unwrap())?,
            _ => Err(DError::UnknownRule)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, From)]
pub struct BinaryExpr(pub BinaryOpCode, pub Box<Expr>, pub Box<Expr>);

impl Serialize for BinaryExpr {
    fn serialize(&self) -> Printable {
        vec![self.1.serialize(), self.0.serialize(), self.2.serialize()].into()
    }
}

#[derive(Debug)]
enum OpOrExpr {
    Op(BinaryOpCode),
    Expr(Expr),
}

impl OpOrExpr {
    pub fn unwrap_op(self) -> BinaryOpCode {
        match self {
            Self::Op(x) => x,
            _ => panic!(),
        }
    }

    pub fn unwrap_ex(self) -> Expr {
        match self {
            Self::Expr(x) => x,
            _ => panic!(),
        }
    }
}

impl BinaryExpr {
    pub fn from_parts(parts: Vec<Pair>) -> Result<Expr, DError> {
        let mut v = vec![];
        for (i, x) in parts.into_iter().enumerate() {
            let x = if i % 2 == 0 {
                OpOrExpr::Expr(Expr::deserialize(x)?)
            } else {
                OpOrExpr::Op(BinaryOpCode::deserialize(x)?)
            };
            v.push(x);
        }
        use BinaryOpCode as B;
        Ok(Self::create_ast(v, BinaryOpCode::tiers()))
    }

    fn create_ast(parts: Vec<OpOrExpr>, ops: &[&[BinaryOpCode]]) -> Expr {
        if parts.len() == 1 {
            return parts.into_iter().next().unwrap().unwrap_ex();
        }
        let mut parts = parts.into_iter();
        let mut new_parts = vec![parts.next().unwrap()];
        let mut found = false;
        while let Some(op) = parts.next() {
            let op = op.unwrap_op();
            let rhs = parts.next().unwrap();
            let this_op = ops[0].iter().find(|&x| *x == op).is_some();
            if this_op {
                found = true;
                let lhs = new_parts.pop().unwrap().unwrap_ex();
                new_parts.push(OpOrExpr::Expr(Expr::Binary(BinaryExpr(
                    op,
                    lhs.into(),
                    rhs.unwrap_ex().into(),
                ))));
            } else {
                new_parts.push(OpOrExpr::Op(op));
                new_parts.push(rhs);
            }
        }
        let ops = match found {
            true => ops,
            false => &ops[1..],
        };
        Self::create_ast(new_parts, ops)
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
mod same_tier {
    use super::*;

    #[test]
    fn power() {
        assert_eq!(
            parse!("2 ^ 3"),
            Ok(Expr::Binary(BinaryExpr(
                BinaryOpCode::Pow,
                2.into(),
                3.into()
            )))
        )
    }

    #[test]
    fn chain_rule() {
        let inner = Expr::Binary(BinaryExpr(BinaryOpCode::Pow, 2.into(), 3.into()));
        assert_eq!(
            parse!("2 ^ 3 @ 4"),
            Ok(Expr::Binary(BinaryExpr(
                BinaryOpCode::Search,
                inner.into(),
                4.into()
            )))
        )
    }
}

#[cfg(test)]
mod diff_tier {
    use super::*;

    #[test]
    fn power() {
        let inner = Expr::Binary(BinaryExpr(BinaryOpCode::Mul, 2.into(), 3.into()));
        assert_eq!(
            parse!("2 * 3 + 4"),
            Ok(Expr::Binary(BinaryExpr(
                BinaryOpCode::Add,
                inner.into(),
                4.into()
            )))
        )
    }

    #[test]
    fn chain_rule() {
        let inner = Expr::Binary(BinaryExpr(BinaryOpCode::Mul, 3.into(), 4.into()));
        assert_eq!(
            parse!("2 + 3 * 4"),
            Ok(Expr::Binary(BinaryExpr(
                BinaryOpCode::Add,
                2.into(),
                inner.into(),
            )))
        )
    }
}
