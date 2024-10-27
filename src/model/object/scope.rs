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
            .try_insert(ident.clone(), value.clone().map(|i| i.src = None))
            .cloned()
            .unwrap_or_else(|e| Bind::constant(Value::err(format!("Could not create variable {} = {:?}, error: {:?}", ident, value, e))))
    }

    pub fn get(&self, ident: String) -> Bind {
        self.0.borrow_mut()
            .get(&ident)
            .cloned()
            .map(|i| i.src = ident.clone())
            .unwrap_or_else(|| Bind::constant(Value::err(format!("Attempted to access unbound local {}", ident))))
    }

    pub fn set(&self, ident: String, val: Bind) {
        // Intended behavior: 
        // 1. If the value under ident doesn't yet exist in scope, set it to val
        // 2. If the value under ident does exist in scope, get it and check what type it is
        // 3. If the value under ident is Opaque and val is also Opaque, dereference both and
        //    bind the underlying Gc<T> directly, preserving memory location so as to propagate
        //    changes to other references to the same gc'd value
        // 4. Otherwise, overwrite the value under ident with val
        
        let mut si = self.0.borrow_mut();
        
        if !si.contains_key(ident.clone()) {
            si.
        }
        
        match val.val {
            Value::Opaque(v) => self.0.borrow_mut().entry(ident).and_modify(|_| val).or_set(val),
            
        }
        self.0.borrow_mut()
            .get(&ident)
            .
    }
}
