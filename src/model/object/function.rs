use std::collections::HashMap;

use crate::model::expression::Expression;
use crate::model::object::Object;

#[derive(Clone, Debug)]
pub struct Function {
    pub args: HashMap<String, Box<dyn Object>>,
    pub expr: Box<Expression>,
}

impl Object for Function {}
