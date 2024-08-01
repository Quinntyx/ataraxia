use super::Object;

use crate::model::object::table::Table;
use crate::model::reference::Value;

impl Object for bool {
    fn index(&self, index: Table) -> Value {
        todo!("FIXME <bool as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Value {
        todo!("FIXME <bool as Object>::get_field has not yet been implemented")
    }
}
