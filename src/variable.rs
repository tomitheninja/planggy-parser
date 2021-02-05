use derive_more::{Constructor, Display, From};

use super::{Expr, Value};

#[derive(Debug, PartialEq, Clone, PartialOrd, From, Display, Constructor)]
pub struct VarName(String);

impl VarName {
    pub fn to_expr(self) -> Expr {
        Value::VarName(self).into()
    }
}

impl From<&str> for VarName {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}
