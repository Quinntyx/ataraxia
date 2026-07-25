use super::Object;

use crate::object::table::Table;
use crate::reference::{Value, Bind};

impl Object for bool {
    fn index(&self, index: Table) -> Bind {
        todo!("FIXME <bool as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <bool as Object>::get_field has not yet been implemented")
    }
}
