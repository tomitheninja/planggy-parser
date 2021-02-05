use super::Expr;
use derive_more::{Constructor, From, Into};

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub enum UnaryCode {
    Plus,
    Neg,
    Abs,
    Parentheses,
    Random,
    Sin,
    Cos,
    Tan,
    Exp,
    Log,
    Arcsin,
    Arccos,
    Arctan,
    Not,
    ToUpper,
    IsWordChar,
    Round,
    ToLower,
    IsDigit,
    ToFloat,
    Floor,
}

#[derive(Debug, PartialEq, From, Clone, Into, Constructor)]
pub struct Unary(UnaryCode, Box<Expr>);

impl Unary {
    pub fn op_code(&self) -> &UnaryCode {
        &self.0
    }

    pub fn to_expr(self) -> Expr {
        self.into()
    }
}

impl From<(UnaryCode, Expr)> for Unary {
    fn from((code, expr): (UnaryCode, Expr)) -> Self {
        (code, Box::new(expr)).into()
    }
}

#[cfg(test)]
macro_rules! test {
    ($test_name:ident: $in:tt -> $code:tt + $expr:expr) => {
        #[test]
        fn $test_name() {
            let parser = crate::Parser::new();
            let result = parser.parse($in).unwrap();
            let expected = Unary::from((crate::UnaryCode::$code, Box::new($expr)));
            assert_eq!(result, expected.to_expr())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    test!(sin_float: "SIN 1.0" -> Sin + 1.0.into());
    test!(chain_rule_float: "SIN COS 1.0" -> Sin + Expr::from(Unary::from((UnaryCode::Cos, Box::new(1.0.into())))));

    test!(parentheses: "(1)" -> Parentheses + 1.into());
    test!(abs: "|1|" -> Abs + 1.into());
}
