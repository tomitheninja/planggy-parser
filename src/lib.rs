#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate pest_derive;
mod traits;

mod ast;

#[allow(unused_imports)]
use pest::Parser;

#[derive(Parser)]
#[grammar = "planggy.pest"]
pub struct PlanggyParser;
