use super::Object;

use crate::model::object::table::Table;
use crate::model::reference::{Bind, Value};

use gc::{Trace, Finalize};

#[derive(Trace, Finalize, Clone, Debug)]
pub struct Unbound;

impl Object for Unbound {
    fn index(&self, index: Table) -> Bind {
        Bind::constant(Value::err(format!("Attempted to index unbound with {:?}", index)))
    }
}
