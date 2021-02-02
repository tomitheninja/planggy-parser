use super::{ast::Constant as C, ConstParser as Parser};

#[cfg(test)]
mod integer {
    use super::*;

    #[test]
    fn one() {
        let parser = Parser::new();
        assert_eq!(parser.parse("1"), Ok(C::Integer(1)));
    }

    #[test]
    fn multi_digit() {
        let parser = Parser::new();
        assert_eq!(parser.parse("1234"), Ok(C::Integer(1234)));
    }

    #[test]
    fn two() {
        let parser = Parser::new();
        assert_eq!(parser.parse("2"), Ok(C::Integer(2)));
    }

    #[test]
    fn signed_one() {
        let parser = Parser::new();
        assert_eq!(parser.parse("+1"), Ok(C::Integer(1)));
    }

    #[test]
    fn signed_negative_one() {
        let parser = Parser::new();
        assert_eq!(parser.parse("-1"), Ok(C::Integer(-1)));
    }
}

#[cfg(test)]
mod float {
    use super::*;

    #[test]
    fn one() {
        let parser = Parser::new();
        assert_eq!(parser.parse("1.0"), Ok(C::Float(1.0)));
    }

    #[test]
    fn two() {
        let parser = Parser::new();
        assert_eq!(parser.parse("2.0"), Ok(C::Float(2.0)));
    }

    #[test]
    fn signed_one() {
        let parser = Parser::new();
        assert_eq!(parser.parse("+1.0"), Ok(C::Float(1.0)));
    }

    #[test]
    fn signed_negative_one() {
        let parser = Parser::new();
        assert_eq!(parser.parse("-1.0"), Ok(C::Float(-1.0)));
    }
}

#[cfg(test)]
mod logical {
    use super::*;

    #[test]
    fn lowercase_true() {
        let parser = Parser::new();
        assert_eq!(parser.parse("igaz"), Ok(C::Boolean(true)));
    }

    #[test]
    fn uppercase_true() {
        let parser = Parser::new();
        assert_eq!(parser.parse("IGAZ"), Ok(C::Boolean(true)));
    }

    #[test]
    fn lowercase_false() {
        let parser = Parser::new();
        assert_eq!(parser.parse("hamis"), Ok(C::Boolean(false)));
    }

    #[test]
    fn uppercase_false() {
        let parser = Parser::new();
        assert_eq!(parser.parse("HAMIS"), Ok(C::Boolean(false)));
    }
}

#[cfg(test)]
mod character {
    use super::*;

    #[test]
    fn normal() {
        let parser = Parser::new();
        assert_eq!(parser.parse("'a'"), Ok(C::Character('a')))
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
        assert_eq!(parser.parse("'\\''"), Ok(C::Character('\'')))
    }

    #[test]
    fn unescaped_backslash() {
        let parser = Parser::new();
        assert!(parser.parse("'\\'").is_err())
    }

    #[test]
    fn escaped_backslash() {
        let parser = Parser::new();
        assert_eq!(parser.parse("'\\\\'"), Ok(C::Character('\\')))
    }
}

#[cfg(test)]
mod string {
    use super::*;

    #[test]
    fn empty() {
        let parser = Parser::new();
        assert_eq!(parser.parse("\"foo\""), Ok(C::String("foo".to_string())))
    }

    #[test]
    fn ascii() {
        let parser = Parser::new();
        assert_eq!(
            parser.parse("\"loremipsum123\""),
            Ok(C::String("loremipsum123".to_string()))
        )
    }

    #[test]
    fn words() {
        let parser = Parser::new();
        assert_eq!(
            parser.parse("\"lorem ipsum\""),
            Ok(C::String("lorem ipsum".to_string()))
        )
    }

    #[test]
    fn emojis() {
        let parser = Parser::new();
        assert_eq!(parser.parse("\"♥\""), Ok(C::String("♥".to_string())))
    }

    #[test]
    fn quotes() {
        let parser = Parser::new();
        assert_eq!(
            parser.parse("\"foo\\\"bar\""),
            Ok(C::String("foo\\\"bar".to_string()))
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
