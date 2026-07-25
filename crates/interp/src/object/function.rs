use super::Object;

use ataraxia_model::expression::Expression;
use crate::object::scope::Scope;
use crate::object::table::Table;
use crate::reference::{Value, Bind, EvaluatedArgument, EvaluatedElement};
use crate::eval::eval;

use std::collections::HashSet;

use gc::{Finalize, Trace};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Function {
    pub args: Vec<EvaluatedArgument>,
    #[unsafe_ignore_trace] // FIXME: This may be an issue, check it if something crashes
    pub expr: Box<Expression>,
    pub scope: Scope,
}

impl Function {
    pub fn new(args: Vec<EvaluatedArgument>, expr: Box<Expression>, scope: Scope) -> Self {
        Self {
            args,
            expr,
            scope
        }
    }
}

impl Object for Function {
    fn index(&self, index: Table) -> Bind {
        todo!("FIXME <Function as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <Function as Object>::get_field has not yet been implemented")
    }

    fn call(&self, args: Vec<EvaluatedElement>) -> Value {
        todo!("Function::call is WIP upstream");
    }
}
