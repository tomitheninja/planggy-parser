#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Constant {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Operation {
    Parentheses(Box<Expression>),
    Not(Box<Expression>),
    Neg(Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Const(Constant),
    Op(Operation),
}
