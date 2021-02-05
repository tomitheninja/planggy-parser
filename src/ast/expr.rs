use derive_more::From;

use super::{Binary, BinaryCode, ConstFloat, ConstInt, Unary, UnaryCode, Value, VarName};

#[derive(Debug, PartialEq, Clone, From)]
pub enum Expr {
    Value(Value),
    Unary(Unary),
    Binary(Binary),
    Index(Box<Expr>, Box<Expr>),
    Slice(Box<Expr>, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn boxed(self) -> Box<Self> {
        self.into()
    }
}

macro_rules! from_const {
    ($T:ident) => {
        impl From<$T> for Expr {
            fn from(x: $T) -> Self {
                Expr::Value(x.into())
            }
        }

        impl From<$T> for Box<Expr> {
            fn from(x: $T) -> Self {
                Expr::from(x).boxed()
            }
        }
    };
}

from_const!(bool);
from_const!(char);
from_const!(ConstInt);
from_const!(ConstFloat);
from_const!(VarName);
from_const!(String);

impl<T> From<(UnaryCode, T)> for Expr
where
    T: Into<Expr>,
{
    fn from((code, e): (UnaryCode, T)) -> Self {
        let unary = Unary::from((code, Box::new(e.into())));
        Expr::Unary(unary)
    }
}

impl<T> From<(UnaryCode, T)> for Box<Expr>
where
    T: Into<Expr>,
{
    fn from((code, e): (UnaryCode, T)) -> Self {
        let unary = Unary::from((code, Box::new(e.into())));
        Expr::Unary(unary).boxed()
    }
}

impl<T, S> From<(BinaryCode, T, S)> for Expr
where
    T: Into<Expr>,
    S: Into<Expr>,
{
    fn from((code, lhs, rhs): (BinaryCode, T, S)) -> Self {
        let binary = Binary::from((code, Box::new(lhs.into()), Box::new(rhs.into())));
        Expr::Binary(binary)
    }
}

impl<T, S> From<(BinaryCode, T, S)> for Box<Expr>
where
    T: Into<Expr>,
    S: Into<Expr>,
{
    fn from((code, lhs, rhs): (BinaryCode, T, S)) -> Self {
        let binary = Binary::from((code, Box::new(lhs.into()), Box::new(rhs.into())));
        Expr::Binary(binary).boxed()
    }
}

#[cfg(test)]
macro_rules! parse {
    ($in:tt) => {{
        let parser = crate::planggy::ExprParser::new();
        parser.parse($in).unwrap()
    }};
}

#[cfg(test)]
macro_rules! test {
    ($test_name:ident: $in:tt -> $lhs:expr ; $code:tt ; $rhs:expr) => {
        #[test]
        fn $test_name() {
            let expected = Binary::from((
                crate::ast::BinaryCode::$code,
                Box::new($lhs),
                Box::new($rhs),
            ));
            assert_eq!(parse!($in), expected.to_expr())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    test!(complex: "1 * 2 + 3 ^ 4 AND 5 = 6" -> {
        let lhs = Expr::from((BinaryCode::Mul, Expr::from(1), Expr::from(2)));
        let rhs = Expr::from((BinaryCode::Pow, Expr::from(3), Expr::from(4)));
        (BinaryCode::Add, lhs, rhs).into()
    } ; And ; {
        Expr::from((BinaryCode::Eq, Expr::from(5), Expr::from(6)))
    });

    #[test]
    fn sign_int() {
        assert_eq!(
            parse!("+1"),
            Unary::from((UnaryCode::Plus, Expr::from(1))).to_expr()
        );
        assert_eq!(
            parse!("-1"),
            Unary::from((UnaryCode::Neg, Expr::from(1))).to_expr()
        );
    }

    #[test]
    fn sign_ws_int() {
        assert_eq!(
            parse!("+ 1"),
            Unary::from((UnaryCode::Plus, Expr::from(1))).to_expr()
        );
        assert_eq!(
            parse!("- 1"),
            Unary::from((UnaryCode::Neg, Expr::from(1))).to_expr()
        );
    }

    #[test]
    fn add_int_int() {
        assert_eq!(
            parse!("1 + 1"),
            Binary::from((BinaryCode::Add, Expr::from(1), Expr::from(1))).to_expr()
        );
    }

    #[test]
    fn sub_int_int() {
        assert_eq!(
            parse!("1 - 1"),
            Binary::from((BinaryCode::Sub, Expr::from(1), Expr::from(1))).to_expr()
        );
    }

    #[test]
    fn add_int_plus_int() {
        assert_eq!(
            parse!("1 + -1"),
            Binary::from((
                BinaryCode::Add,
                Expr::from(1),
                Expr::from((UnaryCode::Neg, Expr::from(1)))
            ))
            .to_expr()
        );
    }

    #[test]
    fn mul_int_plus_int() {
        assert_eq!(
            parse!("1 * -1"),
            Binary::from((
                BinaryCode::Mul,
                Expr::from(1),
                Expr::from((UnaryCode::Neg, Expr::from(1)))
            ))
            .to_expr()
        );
    }
}
