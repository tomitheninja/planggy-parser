#[macro_use]
extern crate pest_derive;

mod raw_expr;
mod stmt;

pub use raw_expr::RawExpr;
pub use stmt::Stmt;
pub use vars_parser::VarName;

#[allow(unused_imports)]
use pest::Parser;
#[derive(Parser)]
#[grammar = "stmts.pest"]
struct StmtsParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if() {
        assert_eq!(
            Stmt::parse(
                r"
            a := 1
            IF 1 = 1 THEN
                b:=2
            END_IF
            "
            ),
            Ok(vec![
                Stmt::Assign("a".into(), "1".into()),
                Stmt::If("1 = 1".into(), vec![Stmt::Assign("b".into(), "2".into())])
            ])
        );
    }

    #[test]
    fn test_while() {
        assert_eq!(
            Stmt::parse(
                r"
            a := 1
            WHILE 1 = 1
                b:=2
                IF 1 THEN
                  PRINT: a+1
                FI
            DONE
            "
            ),
            Ok(vec![
                Stmt::Assign("a".into(), "1".into()),
                Stmt::While(
                    "1 = 1".into(),
                    vec![
                        Stmt::Assign("b".into(), "2".into()),
                        Stmt::If("1".into(), vec![Stmt::Print("a+1".into())])
                    ]
                )
            ])
        );
    }
}
