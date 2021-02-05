use expr_parser::Value;
use std::convert::TryFrom;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VarType {
    Bool,
    Int,
    Float,
    Char,
    String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FromValueError {
    UnknownType,
}

impl TryFrom<&Value> for VarType {
    type Error = FromValueError;
    fn try_from(x: &Value) -> Result<Self, Self::Error> {
        Ok(match x {
            Value::Boolean(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::Char(_) => Self::Char,
            Value::String(_) => Self::String,
            Value::VarName(_) => return Err(FromValueError::UnknownType),
        })
    }
}

impl TryFrom<Value> for VarType {
    type Error = FromValueError;
    fn try_from(x: Value) -> Result<Self, Self::Error> {
        VarType::try_from(&x)
    }
}
