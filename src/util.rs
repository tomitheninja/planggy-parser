static UNESCAPED_SINGLE_QUOTE: &str = "Unescaped single quote";
static EMPTY_CHAR_LITERAL: &str = "Empty char literal";
static UNKNOWN_ESCAPED: &str = r"Unknown escaped character.\
Only {\0, \n, \r, \t } characters are allowed.";
static TOO_LONG_CHARACTER: &str = r"The input is too long to fit in a character.
Consider using a string (with double quotes) instead.";

pub fn char_parser(c: &str) -> Result<char, &'static str> {
    let mut it = c.chars();
    let result = match it.next() {
        Some('\'') => Err(UNESCAPED_SINGLE_QUOTE)?,
        Some('\\') => match it.next() {
            Some('0') => '\0',
            Some('n') => '\n',
            Some('r') => '\r',
            Some('t') => '\t',
            Some('"') => '"', // used by string_parser
            _ => Err(UNKNOWN_ESCAPED)?,
        },
        Some(c) => c,
        None => Err(EMPTY_CHAR_LITERAL)?,
    };
    if it.next().is_none() {
        Ok(result)
    } else {
        Err(TOO_LONG_CHARACTER)
    }
}

static NOT_CLOSED_STR: &str = "String should be closed.";

pub fn string_parser(s: &str) -> Result<String, &'static str> {
    let mut it = s.chars();
    let mut result = String::with_capacity(s.len());
    while let Some(c) = it.next() {
        let c = if c != '\\' {
            c
        } else {
            if let Some(c2) = it.next() {
                let mut s = String::with_capacity(2);
                s.push(c);
                s.push(c2);
                char_parser(&s)?
            } else {
                Err(NOT_CLOSED_STR)?
            }
        };
        result.push(c);
    }
    Ok(result)
}
