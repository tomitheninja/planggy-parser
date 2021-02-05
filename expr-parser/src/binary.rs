use super::Expr;
use derive_more::{Constructor, Display, From, Into};

#[derive(Debug, PartialEq, Copy, Clone, Hash, Display)]
pub enum BinaryCode {
    #[display(fmt = "^")]
    Pow,
    #[display(fmt = "@")]
    Search,
    #[display(fmt = "+")]
    Add,
    #[display(fmt = "-")]
    Sub,
    #[display(fmt = "*")]
    Mul,
    #[display(fmt = "MOD")]
    Mod,
    #[display(fmt = "/")]
    Div,
    #[display(fmt = "DIV")]
    IntDiv,
    #[display(fmt = "=")]
    Eq,
    #[display(fmt = "/=")]
    Ne,
    #[display(fmt = ">")]
    Gt,
    #[display(fmt = ">=")]
    Ge,
    #[display(fmt = "<")]
    Lt,
    #[display(fmt = "<=")]
    Le,
    #[display(fmt = "AND")]
    And,
    #[display(fmt = "OR")]
    Or,
}

#[derive(Debug, PartialEq, From, Clone, Into, Constructor)]
pub struct Binary(BinaryCode, Box<Expr>, Box<Expr>);

impl Binary {
    pub fn op_code(&self) -> &BinaryCode {
        &self.0
    }

    pub fn lhs(&self) -> &Expr {
        &self.1
    }

    pub fn rhs(&self) -> &Expr {
        &self.2
    }

    pub fn to_expr(self) -> Expr {
        self.into()
    }
}

impl From<(BinaryCode, Expr, Expr)> for Binary {
    fn from((code, lhs, rhs): (BinaryCode, Expr, Expr)) -> Self {
        (code, lhs.boxed(), rhs.boxed()).into()
    }
}

#[cfg(test)]
macro_rules! test {
    ($test_name:ident: $in:tt -> $lhs:expr ; $code:tt ; $rhs:expr) => {
        #[test]
        fn $test_name() {
            let parser = crate::Parser::new();
            let result = parser.parse($in).unwrap();
            let expected = Binary::from((crate::BinaryCode::$code, Box::new($lhs), Box::new($rhs)));
            assert_eq!(result, expected.to_expr())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    test!(add_1_1: "1 + 1" -> 1.into() ; Add ; 1.into());
    test!(mul_1_1: "1 * 1" -> 1.into() ; Mul ; 1.into());
    test!(and_true_true: "TRUE AND TRUE" -> true.into() ; And ; true.into());
}
