use super::StatementParser as Parser;

use super::ast::{Operation as O, Statement as S};

#[cfg(test)]
mod statement {
    use super::*;

    #[test]
    fn cout_const() {
        let parser = Parser::new();
        assert_eq!(parser.parse("KI: 1"), Ok(S::Cout(1.into())));
    }

    #[test]
    fn cout_expr() {
        let parser = Parser::new();
        assert_eq!(
            parser.parse("KI: 1 + 2"),
            Ok(S::Cout(O::Add(1.into(), 2.into()).into()))
        );
    }
}
