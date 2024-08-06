use std::collections::HashMap;

use crate::model::reference::{Bind, Value};

use gc::{Trace, Finalize, Gc, GcCell};

#[derive(Clone, Debug, Trace, Finalize)]
pub struct Scope(Gc<GcCell<HashMap<String, Bind>>>);

impl Scope {
    pub fn new() -> Scope {
        Scope(Gc::new(GcCell::new(HashMap::new())))
    }
    
    pub fn subscope(&self) -> Scope {
        Scope(Gc::new(GcCell::new((*self.0).clone().into_inner())))
    }

    pub fn alloc(&self, ident: String, value: Bind) -> Bind {
        self.0.borrow_mut()
            .try_insert(ident.clone(), value.clone())
            .cloned()
            .unwrap_or_else(|e| Bind::constant(Value::err(format!("Could not create variable {} = {:?}, error: {:?}", ident, value, e))))
    }

    pub fn get(&self, ident: String) -> Bind {
        self.0.borrow_mut()
            .get(&ident)
            .cloned()
            .unwrap_or_else(|| Bind::constant(Value::err(format!("Attempted to access unbound local {}", ident))))
    }
}
