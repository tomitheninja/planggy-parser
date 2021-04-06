use fmt::{write, Debug};

use super::{FuncExpr, LiteralExpr, TFloat, TInt, VarName};

#[derive(PartialEq, Clone)]
pub enum Expr {
    Literal(LiteralExpr),
    Function(FuncExpr),
    Parentheses(Box<Expr>),
    Abs(Box<Expr>),
}

macro_rules! from_literal {
    ($T:ty) => {
        impl From<$T> for Expr {
            fn from(x: $T) -> Self {
                Expr::Literal(x.into())
            }
        }
        impl From<$T> for Box<Expr> {
            fn from(x: $T) -> Self {
                Box::new(x.into())
            }
        }
    };
}

from_literal!(char);
from_literal!(&str);
from_literal!(String);
from_literal!(bool);
from_literal!(TInt);
from_literal!(TFloat);
from_literal!(VarName);

impl From<LiteralExpr> for Expr {
    fn from(l: LiteralExpr) -> Self {
        Expr::Literal(l)
    }
}

impl From<LiteralExpr> for Box<Expr> {
    fn from(l: LiteralExpr) -> Self {
        Expr::Literal(l).into()
    }
}

impl From<FuncExpr> for Expr {
    fn from(l: FuncExpr) -> Self {
        Expr::Function(l)
    }
}

impl From<FuncExpr> for Box<Expr> {
    fn from(l: FuncExpr) -> Self {
        Expr::Function(l).into()
    }
}

#[cfg(test)]
macro_rules! parse {
    ($in:expr) => {
        crate::parser::expr($in)
    };
}

#[cfg(test)]
macro_rules! test {
    ($fn_name:ident: $in:tt -> $res:expr) => {
        #[test]
        fn $fn_name() {
            assert_eq!(parse!($in), Ok($res));
        }
    };
}

use std::fmt;
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{:?}", l),
            Self::Parentheses(x) => write!(f, "({:?})", x),
            Self::Abs(x) => write!(f, "|{:?}|", x),
            Self::Function(x) => Debug::fmt(x, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test!(parentheses: "((6))" -> Expr::Parentheses(Box::new(Expr::Parentheses(6.into()))));
}
