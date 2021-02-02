use derive_more::{From, TryInto};

use super::{Constant, Operation};
#[derive(Debug, Clone, PartialEq, PartialOrd, From, TryInto)]
pub enum Expression {
    Const(Constant),
    Op(Operation),
    // TODO:
    // Variable(Variable)
}

impl Expression {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

macro_rules! impl_from {
    ($name:ty) => {
        impl From<$name> for Expression {
            fn from(x: $name) -> Self {
                Expression::Const(x.into())
            }
        }

        impl From<$name> for Box<Expression> {
            fn from(x: $name) -> Self {
                Box::new(x.into())
            }
        }
    };
}

use super::constant::{Float, Int};

impl_from!(Int);
impl_from!(Float);
impl_from!(bool);
impl_from!(char);
impl_from!(String);

#[cfg(test)]
mod traits {
    use super::*;

    #[test]
    fn int_into_expr() {
        assert_eq!(Expression::Const(Constant::Integer(2)), 2.into());
    }

    #[test]
    fn int_into_boxed_expr() {
        assert_eq!(Box::new(Expression::Const(Constant::Integer(2))), 2.into());
    }
}
