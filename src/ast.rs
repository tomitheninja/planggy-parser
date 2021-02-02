#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Constant {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Const(Constant),
}
