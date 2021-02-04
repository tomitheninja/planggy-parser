// use super::{DError, Deserialize, Pair, Printable, Rule, Serialize};
// use derive_more::{Display, From, TryInto};

// use super::{Constant, VariableName};

// #[derive(Debug, PartialEq, From, TryInto, Display)]
// pub enum Value {
//     #[display(fmt = "{}", _0)]
//     Const(Constant),
//     #[display(fmt = "\"{}\"", _0)]
//     VarName(VariableName),
// }

// impl Serialize for Value {
//     fn serialize(&self) -> Printable {
//         vec![match self {
//             Self::Const(c) => c.serialize(),
//             Self::VarName(v) => v.serialize(),
//         }]
//         .into()
//     }
// }

// impl Deserialize for Value {
//     fn deserialize(pair: Pair) -> Result<Self, DError> {
//         use Rule::as R;
//         dbg!(pair.as_rule(), pair.as_str());
//         match pair.as_rule() {
//             R::constant => Constant::deserialize(pair),
//             R::variable_name => VariableName::deserialize(pair),
//             R::value => Self::deserialize(pair.into_inner().next().unwrap()),
//             _ => Err(DError::UnknownRule),
//         }
//     }
// }
