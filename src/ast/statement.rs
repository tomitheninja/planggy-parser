use super::Expression;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Statement {
    Cout(Expression),
}
