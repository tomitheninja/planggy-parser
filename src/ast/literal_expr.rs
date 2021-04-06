use derive_more::From;

use super::Expr;

pub type Int = i32;
pub type Float = f64;

#[derive(Debug, Clone, PartialEq, From)]
pub struct VarName(String);

#[derive(Debug, Clone, PartialEq, From)]
pub enum LiteralExpr {
    Int(Int),
    Float(Float),
    Boolean(bool),
    Char(char),
    String(String),
    Variable(VarName),
}

impl From<&str> for LiteralExpr {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<&str> for VarName {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

// ######### TESTS #########

#[cfg(test)]
macro_rules! parse {
    ($in:expr) => {
        crate::parser::literal($in)
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

#[cfg(test)]
macro_rules! fail {
    ($fn_name:ident: $in:tt) => {
        #[test]
        fn $fn_name() {
            dbg!(parse!($in)).unwrap_err();
        }
    };
}

#[cfg(test)]
mod int {
    test!(zero: r"0" -> 0.into());
    test!(hundred_twenty_three: r"123" -> 123.into());
}

#[cfg(test)]
mod float {
    test!(float: r"123.45" -> 123.45.into());
    test!(float_trialing_f: r"123.45f" -> 123.45.into());
    test!(float_int_trialing_f: r"123f" -> 123.0.into());
}

#[cfg(test)]
mod char {
    test!(normal: r"'c'" -> 'c'.into());
    test!(newline: r"'\n'" -> '\n'.into());
    test!(tab: r"'\t'" -> '\t'.into());
}

#[cfg(test)]
mod string {
    test!(empty: r#""""# -> "".into());
    test!(words: r#""lorem 123 ipsum""# -> "lorem 123 ipsum".into());

    test!(special_character: r#""new\n\t\0\"line""# -> "new\n\t\0\"line".into());

    fail!(backslash_as_last: r#""foo\""#);
}

#[cfg(test)]
mod variable {
    use super::VarName;

    test!(var_a: "a" -> VarName::from("a").into());
    test!(var_many: "many" -> VarName::from("many").into());
    test!(var_many123: "many123" -> VarName::from("many123").into());
    test!(var_a_b: "a_b" -> VarName::from("a_b").into());
    test!(var_underscore: "_" -> VarName::from("_").into());
    fail!(var_digit_first: "0asd");
}
