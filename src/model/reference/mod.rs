use crate::model::object::Object;
use crate::model::object::integer::Integer;
use crate::model::object::range::Range;
use crate::model::object::float::Float64;
use crate::model::object::fraction::Fraction;
use crate::model::object::error::Error;

use gc::{Gc, GcCell, Trace, Finalize};

#[derive(Clone, Trace, Finalize, Debug)]
pub enum Ref {
    Mut(Gc<GcCell<dyn Object>>),
    Const(Gc<GcCell<dyn Object>>),
}

// FIXME: Make sure there are no remaining Value types missing from here
#[derive(Clone, Trace, Finalize, Debug)]
pub enum Value {
    Integer(Integer),
    F64(Float64),
    Range(Range),
    Fraction(Fraction),

    Error(Error),
    Ref(Box<Ref>),

    True,
    False,

    Nil,
}



impl Value {
    pub fn err(str: &str) -> Value {
        Value::Error(Error(String::from(str)))
    }
}

impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            Value::Integer(i) => i.add_value(rhs),
            _ => Value::err("Add has not yet been implemented for this type")
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match &self {
            Value::Integer(i) => i.div_value(rhs),
            _ => Value::err("Add has not yet been implemented for this type")
        }
    }
}
