use super::Object;

use crate::model::object::table::Table;
use crate::model::object::integer::Integer;
use crate::model::reference::Value;

use gc::{Finalize, Trace};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Finalize, Trace)]
pub struct Range(Option<Integer>, Option<Integer>);

impl Object for Range {
    fn index(&self, index: Table) -> Value {
        todo!("FIXME <Range as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Value {
        todo!("FIXME <Range as Object>::get_field has not yet been implemented")
    }
}

impl Range {
    pub fn new(a: Option<i64>, b: Option<i64>) -> Self {
        Self(a.map(|i| i.into()), b.map(|i| i.into()))
    }

    pub fn new_value(a: Option<Value>, b: Option<Value>) -> Value {
        let a = match a.map(|i| i.try_into()).transpose() {
            Ok(a) => a,
            Err(e) => return e,
        };
        
        let b = match b.map(|i| i.try_into()).transpose() {
            Ok(b) => b,
            Err(e) => return e,
        };
        
        Value::Range(Range(a, b))
    }
}
