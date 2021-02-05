use derive_more::{Constructor, Display, From};

#[derive(Debug, PartialEq, Clone, From, Constructor, Display)]
pub struct RawExpr(String);

impl From<&str> for RawExpr {
    fn from(x: &str) -> Self {
        x.to_string().into()
    }
}
