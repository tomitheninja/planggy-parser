use super::{Pair, Expr, VariableName};


pub enum Statement {
    Print(Expr),
    Read(VariableName),
    Assign(VariableName, Expr)
}