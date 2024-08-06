use super::Object;

use crate::model::object::table::Table;
use crate::model::object::fraction::Fraction;
use crate::model::reference::{Value, Bind};

use gc::{Finalize, Trace};
use rug::Integer as BigInt;

#[derive(Trace, Finalize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    #[unsafe_ignore_trace]
    pub internal: BigInt,
}

impl Integer {
    /// Creates an Integer containing a value of 0 and taking no space.
    pub fn new() -> Integer {
        Integer {
            internal: BigInt::new(),
        }
    }
}

impl std::ops::Add for Integer {
    type Output = Integer;
    fn add(self, rhs: Self) -> Self::Output {
        Integer {
            internal: self.internal.clone() + rhs.internal.clone(),
        }
    }
}

impl std::ops::Div for Integer {
    type Output = Fraction;
    fn div(self, rhs: Self) -> Self::Output {
        Fraction {
            numerator: self.internal.clone(),
            denominator: rhs.internal.clone(),
        }
    }
}

impl Object for Integer {
    fn index(&self, index: Table) -> Bind {
        todo!("FIXME <Integer as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <Integer as Object>::get_field has not yet been implemented")
    }

    fn add_value(&self, other: Value) -> Value {
        match &other {
            Value::Integer(i) => Value::Integer(self.clone() + i.clone()),
            _ => Value::err("Attempted to add integer to unsupported type"),
        }
    }

    fn div_value(&self, other: Value) -> Value {
        match &other {
            Value::Integer(i) => Value::Fraction(self.clone() / i.clone()),
            _ => Value::err("Attempted to divide integer by unsupported type"),
        }
    }
}

impl From<i64> for Integer {
    fn from(value: i64) -> Self {
        Self {
            internal: value.into(),
        }
    }
}

impl TryFrom<Value> for Integer {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match &value {
            Value::Integer(i) => Ok(i.clone()),
            _ => Err(Value::err("Attempted to convert non-integral `Value` to `Integer`")),
        }
    }
}

impl std::ops::Add<Fraction> for Integer {
    type Output = Fraction;
    fn add(self, rhs: Fraction) -> Self::Output {
        rhs + self
    }
}
