use super::{DError, Deserialize, Pair, Printable, Rule, Serialize};
use derive_more::{Display, From};

#[derive(Debug, Clone, PartialEq, From, Display)]
pub struct VariableName(String);

impl Serialize for VariableName {
    fn serialize(&self) -> Printable {
        self.0.clone().into()
    }
}

impl Deserialize for VariableName {
    fn deserialize(pair: Pair) -> Result<Self, DError> {
        use Rule as R;
        dbg!(pair.as_rule(), pair.as_str());
        match pair.as_rule() {
            R::variable_name => Ok(pair.as_str().to_string().into()),
            _ => Err(DError::UnknownRule),
        }
    }
}

#[cfg(test)]
macro_rules! parse {
    ($value:expr) => {{
        use crate::{Parser, PlanggyParser};
        Expr::deserialize(
            PlanggyParser::parse(crate::Rule::expr, $value)
                .unwrap()
                .next()
                .unwrap(),
        )
    }};
}

#[cfg(test)]
mod tests {
    use crate::ast::Expr;

    use super::*;

    #[test]
    fn variable_a() {
        assert_eq!(parse!("a"), Ok(Expr::Var("a".to_string().into())))
    }
}
