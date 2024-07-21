use crate::model::object::Object;

#[derive(Clone, Debug, Hash)]
pub enum Primitive {
    Boolean(bool),
    Integer(i64),
    String(String),
}

impl Object for Primitive {}
