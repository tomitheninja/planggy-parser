use super::{Expr, VarName};
use derive_more::{Display, From};

pub type Int = i32;
pub type Float = f64;

#[derive(Debug, PartialEq, Clone, PartialOrd, From, Display)]
pub enum Value {
    Boolean(bool),
    Int(Int),
    Float(Float),
    Char(char),
    String(String),
    VarName(VarName),
}

impl Value {
    pub fn to_expr(self) -> Expr {
        self.into()
    }
}

impl From<&str> for Value {
    fn from(x: &str) -> Self {
        x.to_string().into()
    }
}

#[cfg(test)]
macro_rules! test {
    ($test_name:ident: $in:tt -> $res:expr) => {
        #[test]
        fn $test_name() {
            let parser = crate::planggy::ExprParser::new();
            let result = parser.parse($in).unwrap();
            let expected: crate::ast::Value = $res;
            let expected = expected.to_expr();
            assert_eq!(result, expected)
        }
    };
}

#[cfg(test)]
mod bool {
    test!(uppercase_igaz: "IGAZ" -> true.into());
    test!(lowercase_igaz: "igaz" -> true.into());
    test!(uppercase_true: "TRUE" -> true.into());
    test!(lowercase_true: "true" -> true.into());

    test!(uppercase_hamis: "HAMIS" -> false.into());
    test!(lowercase_hamis: "hamis" -> false.into());
    test!(uppercase_false: "FALSE" -> false.into());
    test!(lowercase_false: "false" -> false.into());
}

#[cfg(test)]
mod int {
    test!(zero: "0" -> 0.into());
    test!(one: "1" -> 1.into());

    test!(hundred: "100" -> 100.into());
}

#[cfg(test)]
mod float {
    test!(zero: "0.0" -> 0.0.into());
    test!(one: "1.0" -> 1.0.into());

    test!(hundred: "100.0" -> 100.0.into());

    test!(int_trailing_f: "1f" -> 1.0.into());
    test!(float_trailing_f: "1.0f" -> 1.0.into());
}

#[cfg(test)]
mod char {
    test!(a: "'a'" -> 'a'.into());
    test!(digit: "'8'" -> '8'.into());

    // TODO: single quote support
    // test!(single_quote: r"'\'''" -> '\''.into());
}

#[cfg(test)]
mod string {
    test!(empty: r#""""# -> "".into());
    test!(one_char: r#""a""# -> "a".into());
    test!(words: r#""natus enim harum""# -> "natus enim harum".into());
    test!(specials: "\"natus\n\0 enim harum\"" -> "natus\n\0 enim harum".into());

    // TODO: double quote support
    // test!(double_quote: r#""natus\" enim harum""# -> "natus\" enim harum".into());
}
