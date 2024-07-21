use std::collections::HashMap;

use crate::model::object::primitive::Primitive;
use crate::model::object::Object;

#[derive(Clone, Debug)]
pub struct Table {
    pub contents: HashMap<Primitive, Box<dyn Object>>,
}

impl Object for Table {}
