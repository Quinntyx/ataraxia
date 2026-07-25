use super::Object;

use crate::object::table::Table;
use crate::reference::{Value, Bind};

impl Object for String {
    fn index(&self, index: Table) -> Bind {
        todo!("FIXME <String as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <String as Object>::get_field has not yet been implemented")
    }
}
