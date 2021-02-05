use super::{variable::VariableWithType, Stmt, VarName};
use derive_more::{Constructor, From};

#[derive(Debug, Clone, PartialEq, Constructor, From)]
pub struct Program {
    name: VarName,
    vars: Vec<VariableWithType>,
    stmts: Vec<Stmt>,
}

impl Program {
    pub fn name(&self) -> &VarName {
        &self.name
    }

    pub fn var_list(&self) -> &Vec<VariableWithType> {
        &self.vars
    }

    pub fn statements(&self) -> &Vec<Stmt> {
        &self.stmts
    }
}

#[cfg(test)]
macro_rules! parse {
    ($in:tt) => {{
        let parser = crate::planggy::ProgramParser::new();
        parser.parse($in).unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use crate::ast::ConstType;

    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            parse!("PROGRAM p PROGRAM_VÉGE"),
            Program {
                name: "p".into(),
                vars: vec![],
                stmts: vec![],
            }
        )
    }

    #[test]
    fn only_vars() {
        let program = "
            PROGRAM p2
                VARIABLES:
                    a, b: Int,
                    c: String
                PROGRAM_VÉGE";
        assert_eq!(
            parse!(program),
            Program {
                name: "p2".into(),
                vars: vec![
                    VariableWithType::new("a".into(), ConstType::Int),
                    VariableWithType::new("b".into(), ConstType::Int),
                    VariableWithType::new("c".into(), ConstType::String)
                ],
                stmts: vec![],
            }
        )
    }

    #[test]
    fn only_stmts() {
        let program = r#"
            PROGRAM p2
                PRINT: "működik!"
                PRINT: "még mindig!"
            PROGRAM_VÉGE"#;
        assert_eq!(
            parse!(program),
            Program {
                name: "p2".into(),
                vars: vec![],
                stmts: vec![
                    Stmt::Print("működik!".to_string().into()),
                    Stmt::Print("még mindig!".to_string().into()),
                ],
            }
        )
    }

    #[test]
    fn vars_and_stmts() {
        let program = r#"
            PROGRAM p2
                VARIABLES:
                    a: Int,
                    b: Float
                READ: a
                READ: b
            PROGRAM_VÉGE"#;
        assert_eq!(
            parse!(program),
            Program {
                name: "p2".into(),
                vars: vec![
                    VariableWithType::new("a".into(), ConstType::Int),
                    VariableWithType::new("b".into(), ConstType::Float)
                ],
                stmts: vec![Stmt::Read("a".into()), Stmt::Read("b".into()),],
            }
        )
    }
}
