use crate::{Parser, RawExpr, VarName};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Assign(VarName, RawExpr),
    Print(RawExpr),
    Read(VarName),
    If(RawExpr, Vec<Stmt>),
    IfElse(RawExpr, Vec<Stmt>, Vec<Stmt>),
    While(RawExpr, Vec<Stmt>),
}

use pest::iterators::Pair as PairTmp;
type Pair<'a> = PairTmp<'a, crate::Rule>;
type PError = pest::error::Error<crate::Rule>;

impl Stmt {
    pub fn parse(s: &str) -> Result<Vec<Self>, PError> {
        let pair = crate::StmtsParser::parse(crate::Rule::input, s)?
            .next()
            .unwrap();
        try_into_stmts(pair)
    }
}

fn try_into_stmt(pair: Pair) -> Result<Stmt, PError> {
    use crate::Rule as R;
    let rule = pair.as_rule();
    let mut pairs = pair.into_inner();
    Ok(match rule {
        R::stmt => try_into_stmt(pairs.next().unwrap())?,
        R::read => Stmt::Read(pairs.as_str().trim().into()),
        R::print => Stmt::Print(pairs.as_str().trim().into()),
        R::assign => {
            let var_name = pairs.next().unwrap().as_str().trim();
            let expr = pairs.next().unwrap().as_str().trim();
            Stmt::Assign(var_name.into(), expr.into())
        }
        R::if_stmt => {
            let expr = pairs.next().unwrap().as_str().trim();
            let inner = try_into_stmts(pairs.next().unwrap()).unwrap();
            Stmt::If(expr.into(), inner)
        }
        R::while_stmt => {
            let expr = pairs.next().unwrap().as_str().trim();
            let inner = try_into_stmts(pairs.next().unwrap());
            Stmt::While(expr.into(), inner.unwrap())
        }
        _ => unimplemented!(),
    })
}

fn try_into_stmts(pair: Pair) -> Result<Vec<Stmt>, PError> {
    assert_eq!(pair.as_rule(), crate::Rule::stmts);
    let mut v = Vec::new();
    for pair in pair.into_inner() {
        dbg!(pair.as_rule());
        let stmt = try_into_stmt(pair)?;
        v.push(stmt);
    }
    Ok(v)
}
