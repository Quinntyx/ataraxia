use crate::model::object::Object;
use crate::model::object::integer::Integer;
use crate::model::object::range::Range;
use crate::model::object::float::Float64;
use crate::model::object::fraction::Fraction;
use crate::model::object::error::Error;

use gc::{Gc, GcCell, Trace, Finalize};

// FIXME: Make sure there are no remaining Value types missing from here
#[derive(Clone, Trace, Finalize, Debug)]
pub enum Value {
    Integer(Integer),
    F64(Float64),
    Range(Range),
    Fraction(Fraction),

    Error(Error),
    Opaque(Box<Gc<GcCell<dyn Object>>>),

    True,
    False,

    Nil,
}

#[derive(Debug, Clone, Copy)]
pub enum BindKind {
    Constant,
    Mutable,
    Once,
}

#[derive(Debug, Clone, Trace, Finalize)]
pub struct Bind {
    #[unsafe_ignore_trace]
    pub kind: BindKind,
    // Implement weak references
    // #[unsafe_ignore_trace]
    // pub src: 
    pub val: Value
}

impl Bind {
    pub fn constant(val: Value) -> Bind {
        Bind {
            kind: BindKind::Constant,
            val,
        }
    }

    pub fn mutable(val: Value) -> Bind {
        Bind {
            kind: BindKind::Mutable,
            val,
        }
    }

    pub fn once(val: Value) -> Bind {
        Bind {
            kind: BindKind::Once,
            val,
        }
    }

    pub fn try_bind(&self, val: Bind) -> Option<Bind> {
        use BindKind::*;
        match (self.kind, val.kind) {
            (Constant, _) => None,
            (Mutable, Constant) => Some(Bind { kind: Mutable, val: val.val.deep_clone(), }),
            (Mutable, Mutable | Once) => Some(Bind { kind: Mutable, val: val.val.clone(), }),
            (Once, Constant) => Some(Bind { kind: Constant, val: val.val.deep_clone(), }),
            (Once, Mutable) => Some(Bind { kind: Constant, val: val.val.clone(), }),
            (Once, Once) => Some(Bind { kind: Once, val: val.val.clone(), }),
        }
    }
}

impl Value {
    pub fn err(str: impl Into<String>) -> Value {
        Value::Error(Error(str.into()))
    }

    pub fn deep_clone(&self) -> Value {
        match self {
            Value::F64(_) | Value::Nil | Value::True | Value::False | Value::Range(_) | Value::Error(_) | Value::Integer(_) | Value::Fraction(_) => self.clone(),
            Value::Opaque(b) => Value::Opaque(b.clone()),
        }
    }
}

impl std::ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match &self {
            Value::Integer(i) => i.add_value(rhs),
            Value::Fraction(i) => i.add_value(rhs),
            Value::F64(i) => i.add_value(rhs),
            Value::Range(i) => i.add_value(rhs),
            Value::True => Integer::from(1).add_value(rhs),
            Value::False => Integer::from(0).add_value(rhs),
            Value::Error(i) => Value::err(format!("Attempted to add error {:?} to {:?}", i, rhs)),
            Value::Nil => Value::err(format!("Attempted to add nil to {:?}", rhs)),
            _ => Value::err("Add has not yet been implemented for this type")
        }
    }
}

impl std::ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match &self {
            Value::Integer(i) => i.sub_value(rhs),
            Value::Fraction(i) => i.sub_value(rhs),
            Value::F64(i) => i.sub_value(rhs),
            Value::Range(i) => i.sub_value(rhs),
            Value::True => Integer::from(1).sub_value(rhs),
            Value::False => Integer::from(0).sub_value(rhs),
            Value::Error(i) => Value::err(format!("Attempted to subtract error {:?} from {:?}", i, rhs)),
            Value::Nil => Value::err(format!("Attempted to subtract nil from {:?}", rhs)),
            _ => Value::err("Subtract has not yet been implemented for this type")
        }
    }
}

impl std::ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match &self {
            Value::Integer(i) => i.div_value(rhs),
            Value::Fraction(i) => i.div_value(rhs),
            Value::F64(i) => i.div_value(rhs),
            Value::Range(i) => i.div_value(rhs),
            Value::True => Integer::from(1).div_value(rhs),
            Value::False => Integer::from(0).div_value(rhs),
            Value::Error(i) => Value::err(format!("Attempted to divide error {:?} by {:?}", i, rhs)),
            Value::Nil => Value::err(format!("Attempted to divide nil by {:?}", rhs)),
            _ => Value::err("Div has not yet been implemented for this type")
        }
    }
}

impl std::ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match &self {
            Value::Integer(i) => i.div_value(rhs),
            Value::Fraction(i) => i.div_value(rhs),
            Value::F64(i) => i.div_value(rhs),
            Value::Range(i) => i.div_value(rhs),
            Value::True => Integer::from(1).div_value(rhs),
            Value::False => Integer::from(0).div_value(rhs),
            Value::Error(i) => Value::err(format!("Attempted to multiply error {:?} by {:?}", i, rhs)),
            Value::Nil => Value::err(format!("Attempted to multiply nil by {:?}", rhs)),
            _ => Value::err("Mul has not yet been implemented for this type"),
        }
    }
}
