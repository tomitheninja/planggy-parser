use super::{Expr, VarName};
use derive_more::From;

#[derive(Debug, Clone, PartialEq, From)]
pub enum Stmt {
    Assign(VarName, Expr),
    Print(Expr),
    Read(VarName),
}

#[cfg(test)]
#[cfg(test)]
macro_rules! parse {
    ($in:tt) => {{
        let parser = crate::planggy::StmtParser::new();
        parser.parse($in).unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use super::super::BinaryCode;

    use super::*;

    #[test]
    fn assign_expr() {
        assert_eq!(
            parse!("a := 1 + 1"),
            Stmt::Assign(
                "a".into(),
                (BinaryCode::Add, Expr::from(1), Expr::from(1)).into()
            )
        )
    }

    #[test]
    fn print() {
        assert_eq!(
            parse!("PRINT: a + 1"),
            Stmt::Print((BinaryCode::Add, VarName::from("a").to_expr(), Expr::from(1)).into())
        )
    }

    #[test]
    fn read() {
        assert_eq!(parse!("READ: a"), Stmt::Read(VarName::from("a")))
    }

    #[test]
    #[should_panic]
    fn read_to_expr() {
        parse!("READ: a + 1");
    }
}
