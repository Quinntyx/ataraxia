use crate::model::expression::Expression as E;
use crate::model::operator::Operator as Op;
use crate::model::object::range::Range;
use crate::model::reference::Value;

pub fn eval(expr: E) -> Value {
    match expr {
        E::Noop => Value::Nil,
        E::Integer(i) => Value::Integer(i.into()),
        E::Bool(b) => if b { Value::True } else { Value::False },
        E::Range(a, b) => Range::new_value(
            a.map(|e| eval(*e)),
            b.map(|e| eval(*e)),
        ),
        E::Block(exprs, rtn) => exprs.into_iter().map(|i| eval(i)).find(|i| matches!(i, Value::Error(_))).unwrap_or_else(|| eval(*rtn)),
        E::Operator { op, left, right } => {
            match op {
                Op::Plus => eval(*left) + eval(*right),
                Op::Divide => eval(*left) / eval(*right),
                _ => Value::err("Operator has not yet been implemented")
            }
        },
        E::Frac(f) => Value::Fraction(f),

        _ => todo!("Not yet implemented"),

    }
}
