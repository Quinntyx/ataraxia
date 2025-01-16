use super::Object;

use crate::model::expression::{Expression, EvaluatedArgument, EvaluatedElement};
use crate::model::object::scope::Scope;
use crate::model::object::table::Table;
use crate::model::reference::{Value, Bind};
use crate::interpreter::eval;

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
        let mut scope = self.scope.subscope();
        let mut named = args.iter().filter(|a| match a {
                EvaluatedElement::KV(_, _) => true,
                _ => false,
            }).collect::<Vec<_>>();

        let named_hash = named.iter().map(|a| match a {
                EvaluatedElement::KV(k, _) => k,
                _ => unreachable!("Already prevented"),
            }).collect::<HashSet<String>>();

        let positional = self.args.iter()
            .filter(|i| !named_hash.contains(
                (|a| {match a {
                    EvaluatedArgument::KV(s, .. ) => s,
                    EvaluatedArgument::V(s, _) => s,
                }})(i)
            )
            .collect::<Vec<_>>();



        self.args.iter().for_each(|a| match a {
            EvaluatedArgument::KV => ,
            EvaluatedArgument::V => unnamed.push(v)
        }
        
    }
}
