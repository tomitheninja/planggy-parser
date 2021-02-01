#[macro_use]
extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub planggy);

pub use planggy::InputParser as Parser;

#[cfg(test)]
mod parser {
    use super::*;

    #[cfg(test)]
    mod constant_expressions {
        use super::{planggy::ConstantExpParser as Parser, *};

        #[cfg(test)]
        mod integer {
            use super::*;

            #[test]
            fn one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("1"), Ok(ast::Constant::Integer(1)));
            }

            #[test]
            fn two() {
                let parser = Parser::new();
                assert_eq!(parser.parse("2"), Ok(ast::Constant::Integer(2)));
            }

            #[test]
            fn signed_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("+1"), Ok(ast::Constant::Integer(1)));
            }

            #[test]
            fn signed_negative_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("-1"), Ok(ast::Constant::Integer(-1)));
            }
        }

        #[cfg(test)]
        mod float {
            use super::*;

            #[test]
            fn one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("1.0"), Ok(ast::Constant::Float(1.0)));
            }

            #[test]
            fn two() {
                let parser = Parser::new();
                assert_eq!(parser.parse("2.0"), Ok(ast::Constant::Float(2.0)));
            }

            #[test]
            fn signed_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("+1.0"), Ok(ast::Constant::Float(1.0)));
            }

            #[test]
            fn signed_negative_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("-1.0"), Ok(ast::Constant::Float(-1.0)));
            }
        }

        #[cfg(test)]
        mod logical {
            use super::*;

            #[test]
            fn lowercase_true() {
                let parser = Parser::new();
                assert_eq!(parser.parse("igaz"), Ok(ast::Constant::Boolean(true)));
            }

            #[test]
            fn uppercase_true() {
                let parser = Parser::new();
                assert_eq!(parser.parse("IGAZ"), Ok(ast::Constant::Boolean(true)));
            }

            #[test]
            fn lowercase_false() {
                let parser = Parser::new();
                assert_eq!(parser.parse("hamis"), Ok(ast::Constant::Boolean(false)));
            }

            #[test]
            fn uppercase_false() {
                let parser = Parser::new();
                assert_eq!(parser.parse("HAMIS"), Ok(ast::Constant::Boolean(false)));
            }
        }
    }
}
