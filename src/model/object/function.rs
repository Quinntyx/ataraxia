use super::Object;

use crate::model::expression::Expression;
use crate::model::object::scope::Scope;
use crate::model::object::table::Table;
use crate::model::reference::{Value, Bind};

use gc::{Finalize, Trace};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Function {
    pub args: Table,
    #[unsafe_ignore_trace] // FIXME: This may be an issue, check it if something crashes
    pub expr: Box<Expression>,
    pub scope: Scope,
}

impl Object for Function {
    fn index(&self, index: Table) -> Bind {
        todo!("FIXME <Function as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <Function as Object>::get_field has not yet been implemented")
    }
}
