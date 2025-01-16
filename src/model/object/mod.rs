pub mod boolean;
pub mod error;
pub mod float;
pub mod function;
pub mod integer;
pub mod range;
pub mod scope;
pub mod string;
pub mod fraction;
pub mod unbound;

pub mod table;

use dyn_clone::DynClone;

use gc::{Finalize, Gc, GcCell, Trace};
use std::fmt::Debug;

use crate::model::reference::{Value, Bind};
use crate::model::expression::EvaluatedElement;

pub trait Object: DynClone + Debug + Trace + Finalize {
    fn get_field(&self, field: String) -> Bind {
        self.index(table::Table::single(Value::Opaque(Box::new(Gc::new(GcCell::new(field)) as _))))
    }

    fn index(&self, index: table::Table) -> Bind;

    fn add_value(&self, other: Value) -> Value {
        Value::err(format!("Attempted to add objects {:?} and {:?} without an add implementation", self, other))
    }

    fn sub_value(&self, other: Value) -> Value {
        Value::err(format!("Attempted to subtract objects {:?} and {:?} without a sub implementation", self, other))
    }
    
    fn mul_value(&self, other: Value) -> Value {
        Value::err(format!("Attempted to multiply objects {:?} and {:?} without a mul implementation", self, other))
    }

    fn div_value(&self, other: Value) -> Value {
        Value::err(format!("Attempted to divide objects {:?} and {:?} without a div implementation", self, other))
    }

    fn call(&self, args: Vec<EvaluatedElement>) -> Value {
        Value::err(format!("Attempted to call object {:?} without a call implementation", self))
    }
}

dyn_clone::clone_trait_object!(Object);
