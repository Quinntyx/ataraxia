use super::Object;

use gc::{Finalize, Trace};

use crate::model::reference::Value;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Finalize, Trace)]
pub struct Error(String);

impl Object for Error {
    fn get_field(&self, field: String) -> Value {
        todo!("FIXME <Error as Object>::get_field has not yet been implemented");
    }

    fn index(&self, index: super::table::Table) -> Value {
        todo!("FIXME <Error as Object>::index has not yet been implemented");
    }
}
