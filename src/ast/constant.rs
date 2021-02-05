use super::Value;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConstType {
    Bool,
    Int,
    Float,
    Char,
    String,
}

impl TryFrom<Value> for ConstType {
    type Error = &'static str;
    fn try_from(x: Value) -> Result<Self, Self::Error> {
        Ok(match x {
            Value::Boolean(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::Char(_) => Self::Char,
            Value::String(_) => Self::String,
            Value::VarName(_) => return Err("Couldn't parse from variable name"),
        })
    }
}

impl TryFrom<&Value> for ConstType {
    type Error = &'static str;
    fn try_from(x: &Value) -> Result<Self, Self::Error> {
        Ok(match x {
            Value::Boolean(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::Char(_) => Self::Char,
            Value::String(_) => Self::String,
            Value::VarName(_) => return Err("Couldn't parse from variable name"),
        })
    }
}
