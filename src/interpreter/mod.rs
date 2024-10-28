use crate::model::expression::Expression as E;
use crate::model::operator::Operator as Op;
use crate::model::object::range::Range;
use crate::model::object::scope::Scope as S;
use crate::model::object::unbound::Unbound;
use crate::model::reference::{Value, Bind as B, BindKind as BK};

use gc::{Gc, GcCell};

pub fn eval(expr: E, scope: S) -> B {
    match expr {
        E::Noop => B::constant(Value::Nil),
        E::Integer(i) => B::mutable(Value::Integer(i.into())),
        E::Bool(b) => B::mutable(if b { Value::True } else { Value::False }),
        E::Range(a, b) => B::mutable(Range::new_value(
            a.map(|e| eval(*e, scope.clone()).val.clone()),
            b.map(|e| eval(*e, scope.clone()).val.clone()),
        )),
        E::Block(exprs, rtn) => {
            let subscope = scope.subscope();
            exprs
                .into_iter()
                .map(|i| eval(i, subscope.clone()))
                .find(|i| matches!(i, B { kind: _, src: _, val: Value::Error(_) }))
                .unwrap_or_else(|| eval(*rtn, subscope.clone()))
        },
        E::Operator { op, left, right } => {
            match op {
                Op::Plus => B::mutable(eval(*left, scope.clone()).val.clone() + eval(*right, scope.clone()).val.clone()),
                Op::Divide => B::mutable(eval(*left, scope.clone()).val.clone() / eval(*right, scope.clone()).val.clone()),
                Op::Minus => B::mutable(eval(*left, scope.clone()).val.clone() - eval(*right, scope.clone()).val.clone()),
                Op::Multiply => B::mutable(eval(*left, scope.clone()).val.clone() * eval(*right, scope.clone()).val.clone()),
                Op::Assign => {
                    let lhs = eval(*left, scope.clone());
                    let Some(src) = lhs.src.clone() else { return B::mutable(Value::err("Cannot assign into a binding with no source")) };
                    B::mutable(scope.clone().set(src, eval(*right, scope.clone())))
                }
                a => B::mutable(Value::err(format!("{:?} operator has not yet been implemented", a))),
            }
        },
        E::Frac(f) => B::mutable(Value::Fraction(f)),
        E::Let(i, v) => scope.alloc(i, eval(*v, scope.clone()).inspect_mut(|mut b| b.kind = BK::Constant)),
        E::Mut(i, v) => scope.alloc(i, eval(*v, scope.clone()).map(|b| match b.kind {
            BK::Constant => B::mutable(Value::err("Attempted coercion of constant to mutable")),
            _ => b
        })),
        E::Identifier(i) => scope.get(i),

        a => todo!("{:?} has not yet been implemented", a),

    }
}
