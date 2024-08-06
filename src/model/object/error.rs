use super::Object;

use gc::{Finalize, Trace};

use crate::model::reference::{Value, Bind};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Finalize, Trace)]
pub struct Error(pub String);

impl Object for Error {
    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <Error as Object>::get_field has not yet been implemented");
    }

    fn index(&self, index: super::table::Table) -> Bind {
        todo!("FIXME <Error as Object>::index has not yet been implemented");
    }
}
