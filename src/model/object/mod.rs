pub mod boolean;
pub mod error;
pub mod float;
pub mod function;
pub mod integer;
pub mod range;
pub mod scope;
pub mod string;
pub mod fraction;

pub mod table;

use dyn_clone::DynClone;

use gc::{Finalize, Gc, GcCell, Trace};
use std::fmt::Debug;

use crate::model::reference::{Value, Ref};

pub trait Object: DynClone + Debug + Trace + Finalize {
    fn get_field(&self, field: String) -> Value {
        self.index(table::Table::single(Value::Ref(Box::new(Ref::Const(Gc::new(GcCell::new(field)) as _)))))
    }

    fn index(&self, index: table::Table) -> Value;
}

dyn_clone::clone_trait_object!(Object);
