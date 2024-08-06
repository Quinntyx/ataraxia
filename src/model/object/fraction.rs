use super::Object;

use crate::model::reference::{Value, Bind};
use crate::model::object::integer::Integer;

use gc::{Finalize, Trace};
use rug::Integer as BigInt;

#[derive(Trace, Finalize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fraction {
    #[unsafe_ignore_trace]
    pub numerator: BigInt,
    #[unsafe_ignore_trace]
    pub denominator: BigInt,
}

impl Object for Fraction {
    fn index(&self, index: super::table::Table) -> Bind {
        todo!("FIXME <Fraction as Object>::index has not yet been implemented");
    }

    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <Fraction as Object>::get_field has not yet been implemented");
    }

    fn add_value(&self, other: Value) -> Value {
        match &other {
            Value::Fraction(i) => Value::Fraction(self.clone() + i.clone()),
            Value::Integer(i) => Value::Fraction(self.clone() + i.clone()),
            _ => Value::err("Attempted to add Fraction to unsupported type"),
        }
    }
}

impl TryFrom<Value> for Fraction {
    type Error = Value;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match &value {
            Value::Fraction(i) => Ok(i.clone()),
            _ => Err(Value::err("Attempted to convert non-fractional `Value` to `Fraction`")),
        }
    }
}

impl std::ops::Add for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Self) -> Self::Output {
        if self.denominator == rhs.denominator {
            Fraction {
                numerator: self.numerator.clone() + rhs.numerator.clone(),
                denominator: self.denominator.clone(),
            }
        } else {
            Fraction {
                numerator: self.numerator.clone() * rhs.denominator.clone() + rhs.numerator.clone() * self.denominator.clone(),
                denominator: self.denominator.clone() * rhs.denominator.clone(),
            }
        }
        
    }
}

impl std::ops::Add<Integer> for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Integer) -> Self::Output {
        Fraction {
            numerator: self.numerator.clone() + rhs.internal.clone() * self.denominator.clone(),
            denominator: self.denominator.clone(),
        }
    }
}
