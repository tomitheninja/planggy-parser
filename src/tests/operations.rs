use super::ExprParser as Parser;

use super::ast::{Expression as E, Operation as O};

#[cfg(test)]
mod operations {
    use super::*;

    #[cfg(test)]
    mod parentheses {
        use super::*;

        #[test]
        fn once() {
            let parser = Parser::new();
            assert_eq!(parser.parse("(123)"), Ok(E::Op(O::Parentheses(123.into()))));
        }

        #[test]
        fn twice() {
            let parser = Parser::new();
            assert_eq!(
                parser.parse("((456))"),
                Ok(E::Op(O::Parentheses(
                    E::Op(O::Parentheses(456.into())).boxed()
                )))
            );
        }

        #[test]
        fn const_item_at() {
            let parser = Parser::new();
            assert_eq!(
                parser.parse("\"foo\"[0]"),
                Ok(E::Op(O::ItemAt("foo".to_string().into(), 0.into())))
            )
        }

        #[test]
        fn const_slice() {
            let parser = Parser::new();
            assert_eq!(
                parser.parse("\"foo\"[0:2]"),
                Ok(E::Op(O::Slice(
                    "foo".to_string().into(),
                    0.into(),
                    2.into()
                )))
            )
        }
    }

    #[cfg(test)]
    mod unary {
        use super::*;

        #[test]
        fn normal_whitespace() {
            let parser = Parser::new();
            assert_eq!(parser.parse("NEM IGAZ"), Ok(E::Op(O::Not(true.into()))))
        }

        #[test]
        fn chain_rule_support() {
            let parser = Parser::new();
            assert!(parser.parse("NEM NEM NEM NEM IGAZ").is_ok())
        }

        #[test]
        fn chain_rule() {
            let parser = Parser::new();
            assert_eq!(
                parser.parse("- NEM IGAZ"),
                Ok(E::Op(O::Neg(E::Op(O::Not(true.into())).boxed())))
            )
        }

        #[test]
        fn missing_whitespace() {
            let parser = Parser::new();
            assert!(parser.parse("NEMIGAZ").is_err());
        }

        #[test]
        fn double_whitespace() {
            let parser = Parser::new();
            assert!(parser.parse("NEM  IGAZ").is_ok());
        }

        #[test]
        fn parentheses() {
            let parser = Parser::new();
            assert_eq!(
                parser.parse("NEM(IGAZ)"),
                Ok(E::Op(O::Not(E::Op(O::Parentheses(true.into())).boxed())))
            );
        }
    }

    #[cfg(test)]
    mod binary {
        use super::*;

        #[test]
        fn add_multiple() {
            let parser = Parser::new();
            let add23 = E::Op(O::Add(2.into(), 3.into()));
            let add234 = E::Op(O::Add(add23.into(), 4.into()));
            let add2345 = E::Op(O::Add(add234.into(), 5.into()));
            assert_eq!(parser.parse("2 + 3 + 4 + 5"), Ok(add2345));
        }

        #[test]
        fn mul_constants() {
            let parser = Parser::new();
            assert_eq!(parser.parse("2 * 3"), Ok(E::Op(O::Mul(2.into(), 3.into()))));
        }

        #[test]
        fn add_constants() {
            let parser = Parser::new();
            assert_eq!(parser.parse("2 + 3"), Ok(E::Op(O::Add(2.into(), 3.into()))));
        }

        #[test]
        fn mul_vs_add_right() {
            let parser = Parser::new();
            assert_eq!(
                parser.parse("2 + 3 * 4"),
                Ok(E::Op(O::Add(
                    2.into(),
                    E::Op(O::Mul(3.into(), 4.into())).boxed()
                )))
            );
        }

        #[test]
        fn mul_vs_add_left() {
            let parser = Parser::new();
            assert_eq!(
                parser.parse("2 * 3 + 4"),
                Ok(E::Op(O::Add(
                    E::Op(O::Mul(2.into(), 3.into())).into(),
                    4.into()
                )))
            );
        }
    }
}
