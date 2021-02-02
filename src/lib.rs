#[macro_use]
extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub planggy);

pub use planggy::InputParser as Parser;

#[cfg(test)]
mod parser {
    use super::*;

    #[cfg(test)]
    mod expressions {
        use super::{planggy::ExprParser as Parser, *};
        use ast::{Constant as C, Expression as E};

        #[cfg(test)]
        mod integer {
            use super::*;

            #[test]
            fn one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("1"), Ok(E::Const(C::Integer(1))));
            }

            #[test]
            fn multi_digit() {
                let parser = Parser::new();
                assert_eq!(parser.parse("1234"), Ok(E::Const(C::Integer(1234))));
            }

            #[test]
            fn two() {
                let parser = Parser::new();
                assert_eq!(parser.parse("2"), Ok(E::Const(C::Integer(2))));
            }

            #[test]
            fn signed_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("+1"), Ok(E::Const(C::Integer(1))));
            }

            #[test]
            fn signed_negative_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("-1"), Ok(E::Const(C::Integer(-1))));
            }
        }

        #[cfg(test)]
        mod float {
            use super::*;

            #[test]
            fn one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("1.0"), Ok(E::Const(C::Float(1.0))));
            }

            #[test]
            fn two() {
                let parser = Parser::new();
                assert_eq!(parser.parse("2.0"), Ok(E::Const(C::Float(2.0))));
            }

            #[test]
            fn signed_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("+1.0"), Ok(E::Const(C::Float(1.0))));
            }

            #[test]
            fn signed_negative_one() {
                let parser = Parser::new();
                assert_eq!(parser.parse("-1.0"), Ok(E::Const(C::Float(-1.0))));
            }
        }

        #[cfg(test)]
        mod logical {
            use super::*;

            #[test]
            fn lowercase_true() {
                let parser = Parser::new();
                assert_eq!(parser.parse("igaz"), Ok(E::Const(C::Boolean(true))));
            }

            #[test]
            fn uppercase_true() {
                let parser = Parser::new();
                assert_eq!(parser.parse("IGAZ"), Ok(E::Const(C::Boolean(true))));
            }

            #[test]
            fn lowercase_false() {
                let parser = Parser::new();
                assert_eq!(parser.parse("hamis"), Ok(E::Const(C::Boolean(false))));
            }

            #[test]
            fn uppercase_false() {
                let parser = Parser::new();
                assert_eq!(parser.parse("HAMIS"), Ok(E::Const(C::Boolean(false))));
            }
        }

        #[cfg(test)]
        mod character {
            use super::*;

            #[test]
            fn normal() {
                let parser = Parser::new();
                assert_eq!(parser.parse("'a'"), Ok(E::Const(C::Character('a'))))
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
                assert_eq!(parser.parse("'\\''"), Ok(E::Const(C::Character('\''))))
            }

            #[test]
            fn unescaped_backslash() {
                let parser = Parser::new();
                assert!(parser.parse("'\\'").is_err())
            }

            #[test]
            fn escaped_backslash() {
                let parser = Parser::new();
                assert_eq!(parser.parse("'\\\\'"), Ok(E::Const(C::Character('\\'))))
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
                    Ok(E::Const(C::String("foo".to_string())))
                )
            }

            #[test]
            fn ascii() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"loremipsum123\""),
                    Ok(E::Const(C::String("loremipsum123".to_string())))
                )
            }

            #[test]
            fn words() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"lorem ipsum\""),
                    Ok(E::Const(C::String("lorem ipsum".to_string())))
                )
            }

            #[test]
            fn emojis() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"♥\""),
                    Ok(E::Const(C::String("♥".to_string())))
                )
            }

            #[test]
            fn quotes() {
                let parser = Parser::new();
                assert_eq!(
                    parser.parse("\"foo\\\"bar\""),
                    Ok(E::Const(C::String("foo\\\"bar".to_string())))
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
