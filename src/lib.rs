#[macro_use]
extern crate lalrpop_util;

#[cfg(test)]
mod tests;

pub mod ast;

lalrpop_mod!(pub planggy);

pub use planggy::InputParser as Parser;
