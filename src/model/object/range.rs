use super::Object;

use crate::model::object::table::Table;
use crate::model::reference::Value;

use gc::{Finalize, Trace};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Finalize, Trace)]
pub struct Range(Option<i64>, Option<i64>);

impl Object for Range {
    fn index(&self, index: Table) -> Value {
        todo!("FIXME <Range as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Value {
        todo!("FIXME <Range as Object>::get_field has not yet been implemented")
    }
}
