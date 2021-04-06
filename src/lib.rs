mod ast;
mod util;

peg::parser! {
  grammar parser() for str {
    use ast::{LiteralExpr, VarName, Expr, FuncExpr};
    use util::{char_parser, string_parser};

    pub rule ws() -> &'input str
        = x:$((" " / "\t" / "\n" / "\r")+) { x }

    pub rule wrapped() -> Expr
        = "(" ws()? e:expr() ws()? ")" { Expr::Parentheses(e.into()) }
        / "|" ws()? e:expr() ws()? "|" { Expr::Abs(e.into()) }

    pub rule expr() -> Expr
        = l:literal() { l.into() }
        / e:wrapped() { e }
        // / f:func() { f.into() }


    pub rule func() -> FuncExpr
        = f:var_name() ws() e:expr() { FuncExpr::new(f, e.into()) }
        / f:$("+" / "-") ws() e:expr() { FuncExpr::new(f.into(), e.into()) }

    pub rule literal() -> LiteralExpr
        = x:(float_e() / integer_e() / character_e() / string_e() / variable_e()) {x}
        pub rule integer_e() -> LiteralExpr
            = x:quiet!{integer()} {x.into()}
            / expected!("integer")
        pub rule float_e() -> LiteralExpr
            = x:quiet!{float()} {x.into()}
            / expected!("float")
        pub rule character_e() -> LiteralExpr
            = x:quiet!{character()} {x.into()}
            / expected!("character")
        pub rule string_e() -> LiteralExpr
            = x:quiet!{string()} {x.into()}
            / expected!("string")
        pub rule variable_e() -> LiteralExpr
            = x:quiet!{var_name()} {x.into()}
            / expected!("variable")


    /// digit
    rule d() -> &'input str = c:$(['0'..='9']) {c}
    rule alpha() -> &'input str = c:$(['a'..='z' | 'A'..='Z']) {c}
    rule w() -> &'input str = c:$(alpha() / d()) {c}
    rule back_slash() -> &'input str = quiet!{"\\"} {"\\"} / expected!("backslash")
    rule any_char() -> &'input str = c:$(quiet!{[_]}) {c} / expected!("any character")

    pub rule integer() -> ast::TInt
      = n:$(d()+){ n.parse().unwrap() }

    pub rule float() -> ast::TFloat
      = n:(float_normal() / float_f()) { n.parse().unwrap() }

        rule float_normal() -> &'input str
            = s:$(d()+ "." d()+) "f"? { s }
        rule float_f() -> &'input str
            = s:$(d()+ ("." d()+)?) "f" { s }

    rule singe_quote() -> &'input str
        = quiet!{"'"} { "'" }
        / expected!("single quote")

    pub rule character() -> char
      = singe_quote() c:$(back_slash()? any_char()) singe_quote() {? match char_parser(c) {
        Ok(c) => Ok(c),
        Err(e) => Err(e),
      }}

    rule double_quote() -> &'input str
        = quiet!{"\""} { "\"" }
        / expected!("double quote")

    pub rule string() -> String
      = double_quote() inner:$(string_inner()*) double_quote() {? match string_parser(inner) {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
      }}
      rule string_inner() -> &'input str
        = s:$(
            (!("\"" / back_slash()) any_char()) // a normal character
            / (back_slash() any_char()) // an escaped character
        ) { s }

    pub rule var_name() -> VarName
        = s:$((alpha()/"_") (w()/"_")* ) { VarName::from(s) }
  }
}
