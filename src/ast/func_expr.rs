use super::{Expr, VarName};
use derive_more::{Constructor, From};

#[derive(Debug, Clone, PartialEq, Constructor, From)]
pub struct FuncExpr {
    func_name: VarName,
    expr: Box<Expr>,
}

#[cfg(test)]
macro_rules! parse {
    ($in:expr) => {
        crate::parser::func($in)
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
mod functions {
    use super::*;

    test!(sin_6: "SIN 6" -> FuncExpr::new("SIN".into(), 6.into()));

    fail!(sin6: "SIN6");
}
