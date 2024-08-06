use super::Object;

use crate::model::object::table::Table;
use crate::model::reference::{Value, Bind};

use ordered_float::OrderedFloat;
use gc::{Trace, Finalize};

#[derive(Hash, Debug, PartialEq, Eq, Trace, Finalize, Clone)]
pub struct Float64 {
    #[unsafe_ignore_trace]
    internal: OrderedFloat<f64>,
}

impl Object for Float64 {
    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <Float64 as Object>::get_field has not yet been implemented")
    }

    fn index(&self, index: Table) -> Bind {
        todo!("FIXME <Float64 as Object>::index has not yet been implemented")
    }
}
