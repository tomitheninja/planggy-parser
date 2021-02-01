#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Constant {
    Integer(i32),
    Float(f64),
    Boolean(bool),
}
