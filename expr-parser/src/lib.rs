#[macro_use]
extern crate lalrpop_util;

mod binary;
mod constant;
mod expr;
mod unary;
mod value;
mod variable;

lalrpop_mod!(pub parsers);

pub use parsers::{
    AtomicParser, ExprParser as Parser, ExprParser, UnaryOPParser, ValueParser, VarNameParser,
};

pub use binary::{Binary, BinaryCode};
pub use constant::ConstType;
pub use expr::Expr;
pub use unary::{Unary, UnaryCode};
pub use value::{Float as TFloat, Int as TInt, Value};
pub use variable::VarName;

impl Expr {
    pub fn parse(
        s: &str,
    ) -> Result<Expr, lalrpop_util::ParseError<usize, lalrpop_util::lexer::Token, &str>> {
        let parser = Parser::new();
        parser.parse(s)
    }
}
