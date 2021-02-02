use derive_more::{From, TryInto};

pub type Int = i32;
pub type Float = f64;

#[derive(Debug, Clone, PartialEq, PartialOrd, From, TryInto)]
pub enum Value {
    Integer(Int),
    Float(Float),
    Boolean(bool),
    Character(char),
    String(String),
}

#[test]
fn int_into_constexpr() {
    assert_eq!(Value::Integer(2), 2.into());
}
