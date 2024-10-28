use std::collections::HashMap;

use crate::model::reference::{Bind, Value, BindKind};

use gc::{Trace, Finalize, Gc, GcCell, GcCellRefMut};

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
            .try_insert(ident.clone(), value.clone().inspect_mut(|mut i| i.src = None))
            .cloned()
            .unwrap_or_else(|e| Bind::constant(Value::err(format!("Could not create variable {} = {:?}, error: {:?}", ident, value, e))))
    }

    pub fn get(&self, ident: String) -> Bind {
        self.0.borrow_mut()
            .get(&ident)
            .cloned()
            .map(|mut i| {
                i.src = Some(ident.clone());
                i
            }).unwrap_or_else(|| Bind::constant(Value::err(format!("Attempted to access unbound local {}", ident))))
    }

    /// Implements mutability checking and shells out to `set_unchecked` or returns a
    /// `Value::Error` if mutability rules prohibit the operation. If the operation
    /// succeeds, returns `Value::Nil`
    ///
    /// * `ident`: the identifier to set a value under in the scope
    /// * `val`: the value to bind
    pub fn set(&self, ident: String, val: Bind) -> Value {
        use BindKind as B;
        let cur_kind = self.0.borrow().get(&ident).map(|i| i.kind).unwrap_or_else(|| val.kind.clone());

        match (cur_kind, val.kind.clone()) {
            (B::Mutable, B::Mutable) => {self.set_unchecked(ident, val); Value::Nil},
            (B::Constant, _) => Value::err("Attempted assignment to a constant"),
            _ => Value::err("Attempted coercion of constant to mutable"),
        }
    }

    
    /// This function sets an identifier to a binding. Does not check whether mutability
    /// rules allows it to occur. 
    ///
    /// Behavior:
    /// 1. If the value under `ident` doesn't yet exist in scope, set it to val
    /// 2. If the value under ident does exist in scope, get it and check what type it is
    /// 3. If the value under ident is Opaque and val is also Opaque, dereference both and
    /// bind the underlying Gc<T> directly, preseving memory location so as to propagate
    /// changes to other references with the same gc'd value (i.e. a Cell),
    /// 4. Otherwise, overwrite the value under `ident` with `val`
    ///
    /// * `ident`: the identifier to set a value under in the scope
    /// * `val`: the value to bind
    pub fn set_unchecked(&self, ident: String, val: Bind) {
        self.0.borrow_mut().entry(ident)
            .and_modify(|e| {
                match (&mut e.val, &val.val) {
                    (Value::Opaque(old), Value::Opaque(new)) => {
                        *old = (*new).clone();
                        e.kind = val.kind;
                    },
                    _ => {
                        *e = val.clone()
                    }
                }
            })
            .or_insert(val);
    }
}
