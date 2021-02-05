# Planggy: expression parser

## Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Usage](#usage)
- [Contributing](../CONTRIBUTING.md)

## About

This crate parses expressions written in the "planggy" language into [abstract syntax tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree).

## Getting Started

Clone this repository to your local machine.

### Prerequisites

- [rustup](https://rustup.rs/)

### Installing

Add this crate to your cargo.toml

```toml
[dependencies]
expr-parser = { path = "./expr-parser" }
```

## Usage

Add `use expr_parser::Expr` to the top of your rust source file

You can now parse string slices using `Expr::parse`

For more details, check [the example](./examples/simple.rs)
