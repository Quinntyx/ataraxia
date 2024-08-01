use super::Object;

use crate::model::object::table::Table;
use crate::model::reference::Value;

use gc::{Finalize, Trace};
use rug::Integer as BigInt;

#[derive(Trace, Finalize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    #[unsafe_ignore_trace]
    internal: BigInt,
}

impl Integer {
    /// Creates an Integer containing a value of 0 and taking no space.
    pub fn new() -> Integer {
        Integer {
            internal: BigInt::new(),
        }
    }
}

impl Object for Integer {
    fn index(&self, index: Table) -> Value {
        todo!("FIXME <Integer as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Value {
        todo!("FIXME <Integer as Object>::get_field has not yet been implemented")
    }
}
