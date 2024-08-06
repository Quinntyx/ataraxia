use crate::model::expression::Expression as E;
use crate::model::operator::Operator as Op;
use crate::model::object::range::Range;
use crate::model::object::scope::Scope as S;
use crate::model::object::unbound::Unbound;
use crate::model::reference::{Value, Bind as B};

use gc::{Gc, GcCell};

pub fn eval(expr: E, scope: S) -> B {
    match expr {
        E::Noop => B::constant(Value::Nil),
        E::Integer(i) => B::constant(Value::Integer(i.into())),
        E::Bool(b) => B::constant(if b { Value::True } else { Value::False }),
        E::Range(a, b) => B::constant(Range::new_value(
            a.map(|e| eval(*e, scope.clone()).val.clone()),
            b.map(|e| eval(*e, scope.clone()).val.clone()),
        )),
        E::Block(exprs, rtn) => {
            let subscope = scope.subscope();
            exprs
                .into_iter()
                .map(|i| eval(i, subscope.clone()))
                .find(|i| matches!(i, B { kind: _, val: Value::Error(_) }))
                .unwrap_or_else(|| eval(*rtn, subscope.clone()))
        },
        E::Operator { op, left, right } => {
            match op {
                Op::Plus => B::constant(eval(*left, scope.clone()).val.clone() + eval(*right, scope.clone()).val.clone()),
                Op::Divide => B::constant(eval(*left, scope.clone()).val.clone() / eval(*right, scope.clone()).val.clone()),
                Op::Minus => B::constant(eval(*left, scope.clone()).val.clone() - eval(*right, scope.clone()).val.clone()),
                Op::Multiply => B::constant(eval(*left, scope.clone()).val.clone() * eval(*right, scope.clone()).val.clone()),
                Op::Assign => dbg!(eval(*left, scope.clone())) 
                    HERE Implement weak references OR place expressions
                    .try_bind(dbg!(eval(*right, scope.clone())))
                    .unwrap_or_else(|| B::constant(Value::err("Attempted to assign into a constant binding"))),
                _ => B::constant(Value::err("Operator has not yet been implemented")),
            }
        },
        E::Frac(f) => B::constant(Value::Fraction(f)),
        E::Let(f) => scope.alloc(f, B::once(Value::Opaque(Box::new(Gc::new(GcCell::new(Unbound)))))),
        E::Identifier(i) => scope.get(i),

        _ => todo!("Not yet implemented"),

    }
}
