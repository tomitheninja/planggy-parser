use derive_more::{From, TryInto};

pub type Int = i32;
pub type Float = f64;

#[derive(Debug, Clone, PartialEq, PartialOrd, From, TryInto)]
pub enum Constant {
    Integer(Int),
    Float(Float),
    Boolean(bool),
    Character(char),
    String(String),
}

#[test]
fn int_into_constexpr() {
    assert_eq!(Constant::Integer(2), 2.into());
}
