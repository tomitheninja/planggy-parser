#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub planggy);

pub use planggy::InputParser as Parser;

#[cfg(test)]
mod parser {
    use super::Parser;

    #[test]
    fn hello_world() {
        assert_eq!(Parser::new().parse("HELLO"), Ok("WORLD".to_string()));
    }
}
