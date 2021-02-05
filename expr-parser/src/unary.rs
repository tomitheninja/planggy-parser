use super::Expr;
use derive_more::{Constructor, Display, From, Into};

#[derive(Debug, PartialEq, Copy, Clone, Hash, Display)]
pub enum UnaryCode {
    Abs,
    Parentheses,
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Neg,
    #[display(fmt = "RND")]
    Random,
    #[display(fmt = "SIN")]
    Sin,
    #[display(fmt = "COS")]
    Cos,
    #[display(fmt = "TAN")]
    Tan,
    #[display(fmt = "EXP")]
    Exp,
    #[display(fmt = "LOG")]
    Log,
    #[display(fmt = "ARCSIN")]
    Arcsin,
    #[display(fmt = "ARCCOS")]
    Arccos,
    #[display(fmt = "ARCTAN")]
    Arctan,
    #[display(fmt = "NOT")]
    Not,
    #[display(fmt = "UPPER")]
    ToUpper,
    #[display(fmt = "CHAR")]
    IsWordChar,
    #[display(fmt = "ROUND")]
    Round,
    #[display(fmt = "LOWER")]
    ToLower,
    #[display(fmt = "DIGIT")]
    IsDigit,
    #[display(fmt = "FLOAT")]
    ToFloat,
    #[display(fmt = "INT")]
    ToInt,
}

#[derive(Debug, PartialEq, From, Clone, Into, Constructor)]
pub struct Unary(UnaryCode, Box<Expr>);

impl Unary {
    pub fn op_code(&self) -> &UnaryCode {
        &self.0
    }

    pub fn rhs(&self) -> &Expr {
        &self.1
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
