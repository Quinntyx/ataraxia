use crate::model::object::Object;

#[derive(Clone, Debug)]
pub struct Float(pub f64);

impl Object for Float {}
