use super::{
    binary, unary, BinaryExpr, Constant, DError, Deserialize, Pair, Printable, Rule, Serialize,
    UnaryExpr, VariableName, Wrapper,
};
use derive_more::From;

#[derive(Debug, Clone, PartialEq, From)]
pub enum Expr {
    Const(Constant),
    Wrapped(Wrapper),
    Var(VariableName),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

macro_rules! impl_from_const {
    ($name:ty) => {
        impl From<$name> for Expr {
            fn from(x: $name) -> Self {
                Expr::Const(x.into())
            }
        }

        impl From<$name> for Box<Expr> {
            fn from(x: $name) -> Self {
                Box::new(x.into())
            }
        }
    };
}

impl_from_const!(super::constant::Int);
impl_from_const!(super::constant::Float);
impl_from_const!(char);
impl_from_const!(String);
impl_from_const!(bool);

impl<T> From<(unary::UnaryOpCode, T)> for Expr
where
    T: Into<Expr>,
{
    fn from(x: (unary::UnaryOpCode, T)) -> Self {
        Expr::Unary((x.0, x.1.into()).into())
    }
}

impl<T, S> From<(binary::BinaryOpCode, T, S)> for Expr
where
    T: Into<Expr>,
    S: Into<Expr>,
{
    fn from(x: (binary::BinaryOpCode, T, S)) -> Self {
        Expr::Binary(BinaryExpr(x.0, Box::new(x.1.into()), Box::new(x.2.into())))
    }
}

impl Serialize for Expr {
    fn serialize(&self) -> Printable {
        use Expr as E;
        let x = match self {
            E::Binary(b) => b.serialize(),
            E::Const(c) => c.serialize(),
            E::Unary(u) => u.serialize(),
            E::Var(v) => v.serialize(),
            E::Wrapped(w) => w.serialize(),
        };
        vec![x].into()
    }
}

impl Deserialize for Expr {
    fn deserialize(pair: Pair) -> Result<Self, DError> {
        use Rule::{boolean, character, constant, float, number, string};
        dbg!(pair.as_rule(), pair.as_str());
        match pair.as_rule() {
            Rule::unary => Ok(UnaryExpr::deserialize(pair)?.into()),
            // The current implemenetation of unary operations requires this
            constant | boolean | character | float | number | string => {
                Ok(Constant::deserialize(pair)?.into())
            }
            Rule::wrapper | Rule::parentheses | Rule::abs | Rule::index | Rule::slice => {
                Ok(Self::Wrapped(Wrapper::deserialize(pair)?))
            }
            // binary or self
            Rule::expr => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let is_binary = pairs.peek().is_some();
                if !is_binary {
                    Self::deserialize(first)
                } else {
                    let mut v = vec![first];
                    while let Some(pair) = pairs.next() {
                        v.push(pair);
                    }
                    BinaryExpr::from_parts(v)
                }
            }
            Rule::variable_name => Ok(Self::Var(VariableName::deserialize(pair)?)),
            _ => Err(DError::UnknownRule),
        }
    }
}

#[cfg(test)]
macro_rules! parse {
    ($value:expr) => {{
        use crate::{ast::Expr, traits::Deserialize, Parser, PlanggyParser};
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

    use crate::ast::unary::UnaryOpCode;

    use super::super::{BinaryOpCode, Expr, VariableName};

    #[test]
    fn parse_macro() {
        assert_eq!(parse!("1"), Ok(Expr::Const(1.into())))
    }

    #[test]
    fn expr() {
        let one: Expr = 1.into();
        let two: Expr = 2.into();
        let three: Expr = 3.into();
        let var_a: VariableName = "a".to_string().into();
        let var_a: Expr = var_a.into();
        let var_b: VariableName = "b".to_string().into();
        let var_b: Expr = var_b.into();
        let t1: Expr = (BinaryOpCode::Mul, var_a, two).into();
        let t2: Expr = (BinaryOpCode::Mul, three, var_b).into();
        let lhs: Expr = (BinaryOpCode::Add, t1, t2).into();
        let result: Expr = (BinaryOpCode::Eq, lhs, one).into();

        assert_eq!(parse!("a*2+3*b=1"), Ok(result))
    }

    #[test]
    fn expr_with_unary() {
        let two: Expr = 2.into();
        let var_a: VariableName = "a".to_string().into();
        let sin_a: Expr = (UnaryOpCode::Sin, var_a).into();

        assert_eq!(
            parse!("sin a * 2"),
            Ok((BinaryOpCode::Mul, sin_a, two).into())
        );
    }

    #[test]
    fn whitespace_inside() {
        let one: Expr = 1.into();
        let two: Expr = 2.into();
        let three: Expr = 3.into();
        let var_a: VariableName = "a".to_string().into();
        let var_a: Expr = var_a.into();
        let var_b: VariableName = "b".to_string().into();
        let var_b: Expr = var_b.into();
        let t1: Expr = (BinaryOpCode::Mul, var_a, two).into();
        let t2: Expr = (BinaryOpCode::Mul, three, var_b).into();
        let lhs: Expr = (BinaryOpCode::Add, t1, t2).into();
        let result: Expr = (BinaryOpCode::Eq, lhs, one).into();

        assert_eq!(parse!("a  * 2 + 3 * b = 1"), Ok(result))
    }

    #[test]
    fn serialize() {
        use crate::traits::Serialize;
        let result = parse!("a  * 2 +  3 * b = 1").unwrap();

        assert_eq!(String::from(result.serialize()), "a*2+3*b=1".to_string())
    }

    #[test]
    fn serialize_unary() {
        use crate::traits::Serialize;
        let result = parse!("NEM(IGAZ) = HAMIS").unwrap();

        assert_eq!(
            String::from(result.serialize()),
            "NEM (IGAZ)=HAMIS".to_string()
        )
    }
}
