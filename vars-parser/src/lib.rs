#[macro_use]
extern crate lalrpop_util;

mod inner_type;
mod name;
mod variable;

lalrpop_mod!(pub parsers);

pub use inner_type::{FromValueError, VarType};
pub use name::VarName;
pub use variable::Variable;

pub use parsers::{
    MaybeVariablesParser, MaybeVariablesParser as Parser, VarDeclarationParser, VarNameParser,
    VariablesParser, VariablesParser as NotEmptyParser,
};

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($test_name:ident using $parser:tt: $in:tt -> $res:expr) => {
            #[test]
            fn $test_name() {
                let parser = $parser::new();
                let result = parser.parse($in).unwrap();
                assert_eq!(result, $res)
            }
        };
    }

    test!(var_a using VarNameParser: "a" -> VarName::from("a"));

    test!(var_foo123 using VarNameParser: "_foo123" -> VarName::from("_foo123"));

    test!(single_var using VarDeclarationParser: "a: Int" -> vec![Variable::new("a".into(), VarType::Int)]);

    test!(same_type using VarDeclarationParser: "a, b: Int" -> vec![Variable::new("a".into(), VarType::Int), Variable::new("b".into(), VarType::Int)]);

    test!(diff_type using VarDeclarationParser: "a: Int, b: Float" -> vec![Variable::new("a".into(), VarType::Int), Variable::new("b".into(), VarType::Float)]);

    test!(all_kind_of using VarDeclarationParser: "a, b: Int, c: Float, d: String, e, f: Bool, g: Char, h: Char" -> vec![
        Variable::new("a".into(), VarType::Int),
        Variable::new("b".into(), VarType::Int),
        Variable::new("c".into(), VarType::Float),
        Variable::new("d".into(), VarType::String),
        Variable::new("e".into(), VarType::Bool),
        Variable::new("f".into(), VarType::Bool),
        Variable::new("g".into(), VarType::Char),
        Variable::new("h".into(), VarType::Char),
    ]);

    test!(full_declaration using VariablesParser: "VARIABLES: a, b: Int, c: Float, d: String, e, f: Bool, g: Char, h: Char" -> vec![
        Variable::new("a".into(), VarType::Int),
        Variable::new("b".into(), VarType::Int),
        Variable::new("c".into(), VarType::Float),
        Variable::new("d".into(), VarType::String),
        Variable::new("e".into(), VarType::Bool),
        Variable::new("f".into(), VarType::Bool),
        Variable::new("g".into(), VarType::Char),
        Variable::new("h".into(), VarType::Char),
    ]);

    test!(full_declaration2 using MaybeVariablesParser: "VARIABLES: a, b: Int, c: Float, d: String, e, f: Bool, g: Char, h: Char" -> vec![
        Variable::new("a".into(), VarType::Int),
        Variable::new("b".into(), VarType::Int),
        Variable::new("c".into(), VarType::Float),
        Variable::new("d".into(), VarType::String),
        Variable::new("e".into(), VarType::Bool),
        Variable::new("f".into(), VarType::Bool),
        Variable::new("g".into(), VarType::Char),
        Variable::new("h".into(), VarType::Char),
    ]);

    test!(empty_declaration using MaybeVariablesParser: "" -> vec![]);
}
