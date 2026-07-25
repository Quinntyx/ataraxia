use super::Object;
use crate::object::integer::Integer;
use crate::object::table::Table;
use crate::reference::{Bind, Value};
pub use ataraxia_model::fraction::Fraction;

impl Object for Fraction {
    fn index(&self, index: Table) -> Bind {
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

impl std::ops::Add<Integer> for Fraction {
    type Output = Fraction;
    fn add(self, rhs: Integer) -> Self::Output {
        Fraction {
            numerator: self.numerator.clone() + rhs.internal.clone() * self.denominator.clone(),
            denominator: self.denominator.clone(),
        }
    }
}