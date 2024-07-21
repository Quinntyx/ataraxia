use std::collections::HashMap;

use crate::model::object::float::Float;
use crate::model::object::function::Function;
use crate::model::object::primitive::Primitive;
use crate::model::object::table::Table;
use crate::model::object::Object;
use crate::model::operator::Operator;

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Primitive),
    Float(Float),
    Block(Vec<Expression>, Box<Expression>),

    Conditional {
        cond: Box<Expression>,
        eval: Box<Expression>,
    },
    If {
        // this should always be Conditional
        cases: Vec<Expression>,
    },

    For {
        variable: String,
        object: Box<Expression>,
    },
    Continue,
    Break {
        ret: Option<Box<Expression>>,
    },

    Operator {
        op: Operator,
        left: Option<Box<Expression>>,
        right: Option<Box<Expression>>,
    },
    Access {
        left: Box<Expression>,
        right: String,
    },
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },

    Identifier(String),
    OptionalIdentifier(String),

    Call {
        callable: Box<Expression>,
        arguments: HashMap<String, Expression>,
    },

    Fn(String, Function),
    Return {
        ret: Option<Box<Expression>>,
    },

    Let(String),
    Mut(String),

    Table(Table),
}

impl Object for Expression {}

impl Expression {
    pub fn op_unary_minus(r: Expression) -> Self {
        Expression::Operator {
            op: Operator::Minus,
            left: None,
            right: Some(Box::new(r)),
        }
    }

    pub fn op_minus(l: Expression, r: Expression) -> Self {
        Expression::Operator {
            op: Operator::Minus,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }

    pub fn op_plus(l: Expression, r: Expression) -> Self {
        Expression::Operator {
            op: Operator::Plus,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }

    pub fn op_multiply(l: Expression, r: Expression) -> Self {
        Expression::Operator {
            op: Operator::Multiply,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }

    pub fn op_divide(l: Expression, r: Expression) -> Self {
        Expression::Operator {
            op: Operator::Divide,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }

    pub fn cond(l: Expression, r: Expression) -> Self {
        Expression::Conditional {
            cond: Box::new(l),
            eval: Box::new(r),
        }
    }
}
