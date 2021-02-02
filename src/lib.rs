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

        #[cfg(test)]
        mod character {
            use ast::Constant;

            use super::*;

            #[test]
            fn normal() {
                let parser = Parser::new();
                assert_eq!(parser.parse("'a'"), Ok(Constant::Character('a')))
            }

            #[test]
            fn invalid_single_quote() {
                let parser = Parser::new();
                assert!(parser.parse("'''").is_err());
            }

            #[test]
            fn empty() {
                let parser = Parser::new();
                assert!(parser.parse("''").is_err());
            }

            #[test]
            fn single_quote() {
                let parser = Parser::new();
                assert_eq!(parser.parse("'\\''"), Ok(Constant::Character('\'')))
            }

            #[test]
            fn unescaped_backslash() {
                let parser = Parser::new();
                assert!(parser.parse("'\\'").is_err())
            }

            #[test]
            fn escaped_backslash() {
                let parser = Parser::new();
                assert_eq!(parser.parse("'\\\\'"), Ok(Constant::Character('\\')))
            }
        }

        #[cfg(test)]
        mod string {
            use super::*;

            #[test]
            fn empty() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"foo\""),
                    Ok(ast::Constant::String("foo".to_string()))
                )
            }

            #[test]
            fn ascii() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"loremipsum123\""),
                    Ok(ast::Constant::String("loremipsum123".to_string()))
                )
            }

            #[test]
            fn words() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"lorem ipsum\""),
                    Ok(ast::Constant::String("lorem ipsum".to_string()))
                )
            }

            #[test]
            fn emojis() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"♥\""),
                    Ok(ast::Constant::String("♥".to_string()))
                )
            }

            #[test]
            fn quotes() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"foo\\\"bar\""),
                    Ok(ast::Constant::String("foo\\\"bar".to_string()))
                )
            }

            #[test]
            fn backslash_inside() {
                let parser = Parser::new();
                assert!(parser.parse("\"fo\\o\"").is_ok());
            }

            #[test]
            fn backslash_as_last() {
                let parser = Parser::new();
                assert!(parser.parse("\"fo\\\"").is_err());
            }

            #[test]
            fn escaped_backslash_as_last() {
                let parser = Parser::new();
                assert!(parser.parse("\"fo\\\\\"").is_ok());
                assert!(parser.parse("\"\\\\\"").is_ok());
            }
        }
    }
}
