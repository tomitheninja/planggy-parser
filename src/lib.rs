#[macro_use]
extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub planggy);

#[cfg(test)]
mod tests;
