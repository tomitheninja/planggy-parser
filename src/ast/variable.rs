use derive_more::{Constructor, Display, From};

use super::{ConstType, Expr, Value};

#[derive(Debug, PartialEq, Clone, PartialOrd, From, Display)]
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

#[derive(Debug, PartialEq, Clone, From, Constructor)]
pub struct VariableWithType {
    name: VarName,
    its_type: ConstType,
}

impl VariableWithType {
    pub fn name(&self) -> &VarName {
        &self.name
    }

    pub fn its_type(&self) -> ConstType {
        self.its_type
    }
}
