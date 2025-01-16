use super::Object;

use std::collections::HashMap;

use crate::model::object::integer::Integer;
use crate::model::object::range::Range;
use crate::model::object::float::Float64;
use crate::model::object::fraction::Fraction;
use crate::model::object::error::Error;
use crate::model::reference::{Value, Bind};

use gc::{Finalize, Trace};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Table(HashMap<Key, (Value, Value)>);

#[derive(Hash, Clone, Debug, Trace, Finalize, Eq, PartialEq)]
enum Key {
    Integer(Integer),
    F64(Float64),
    Range(Range),
    Fraction(Fraction),
    Ref(usize),
    Error(Error),
    True,
    False,
}

impl From<Value> for Key {
    fn from(value: Value) -> Self {
        match &value {
            Value::Opaque(b) => Key::Ref(b.as_ref() as *const _ as usize),
            Value::Range(r) => Key::Range(r.clone()),
            Value::F64(f) => Key::F64(f.clone()),
            Value::Fraction(f) => Key::Fraction(f.clone()),
            Value::Integer(i) => Key::Integer(i.clone()),
            Value::Error(e) => Key::Error(e.clone()),
            Value::True => Key::True,
            Value::False => Key::False,
            Value::Nil => unreachable!("Cannot construct a key from nil, this should have been caught by an earlier check and is an unintended panic"),
        }
    }
}

impl Table {
    pub fn new() -> Table {
        Table(HashMap::new())
    }

    pub fn single(value: Value) -> Table {
        let mut table = Table::new();
        table.insert(Value::Integer(Integer::new()), value);
        table
    }

    pub fn insert(&mut self, key: Value, value: Value) {
        self.0
            .insert(key.clone().into(), (key, value));
    }

    pub fn push_list(&mut self, value: Value) {
        let idx = self.0
            .values()
            // Note: Inefficiency here
            .filter_map(|(k, v)| match k { Value::Integer(i) => Some(i.clone()), _ => None, })
            .max()
            .unwrap_or(Integer::from(0));

        self.insert(Value::Integer(Integer::from(idx + Integer::from(1))), value);
    }
}

impl Object for Table {
    fn index(&self, index: Table) -> Bind {
        todo!("FIXME <Table as Object>::index has not yet been implemented")
    }

    fn get_field(&self, field: String) -> Bind {
        todo!("FIXME <Table as Object>::get_field has not yet been implemented")
    }
}
