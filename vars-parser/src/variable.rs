use crate::{VarName, VarType};
use derive_more::{Constructor, From};

#[derive(Debug, PartialEq, Clone, Constructor, From)]
pub struct Variable {
    name: VarName,
    its_type: VarType,
}
