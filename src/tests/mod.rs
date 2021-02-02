use super::{
    ast,
    planggy::{ConstParser, ExprParser},
};

mod constants;
mod operations;

#[cfg(test)]
mod expression {
    use super::{ast::Expression as E, ExprParser as Parser};

    #[test]
    fn const_as_expression() {
        let parser = Parser::new();
        assert_eq!(parser.parse("123"), Ok(E::Const(123.into())));
    }
}
