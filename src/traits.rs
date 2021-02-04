use derive_more::{Display, Error, From};
use std::{fmt, ops::Add};

use pest::iterators::Pair;

#[derive(Debug, PartialEq, From)]
pub enum Printable {
    Str(String),
    Vec(Vec<Printable>),
}

impl From<Printable> for String {
    fn from(x: Printable) -> Self {
        match x {
            Printable::Str(s) => s,
            Printable::Vec(v) => v
                .into_iter()
                .fold(String::new(), |s, p| s + &String::from(p)),
        }
    }
}

pub trait Serialize: fmt::Debug {
    fn serialize(&self) -> Printable;
}

#[derive(Debug, PartialEq, Display, Error)]
pub enum DError {
    UnknownRule,
    ConstValueTooLarge,
    UnknownExpression,
}

pub trait Deserialize: Sized {
    fn deserialize(pair: Pair<crate::Rule>) -> Result<Self, DError>;
}
