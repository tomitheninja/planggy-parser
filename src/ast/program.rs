use super::{Pair, VariableName, Statement};


pub struct ProgramName(String);

pub struct Program {
    name: ProgramName,
    vars: Vec<VariableName>,
    statements: Vec<Statement>,
}