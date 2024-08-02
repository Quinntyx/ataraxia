use crate::model::operator::Operator;
use crate::model::object::fraction::Fraction;

#[derive(Clone, Debug)]
pub enum Element {
    KV((Expression, Expression)),
    V(Expression),
}

#[derive(Clone, Debug)]
pub enum Expression {
    Noop,
    Integer(i64),
    Bool(bool),
    // FIXME?: Inclusive Ranges implemented as adding 1 arbitrarily, perhaps should support custom
    // types instead and have a special inclusive behavior instead of expecting integer?
    Range(Option<Box<Expression>>, Option<Box<Expression>>),
    Err(String),
    Nil(String),
    String(String),
    Frac(Fraction),
    Block(Vec<Expression>, Box<Expression>),

    Conditional {
        cond: Box<Expression>,
        eval: Box<Expression>,
    },
    If(
        // this should always be Conditional
        Vec<Expression>,
    ),

    For {
        variable: String,
        object: Box<Expression>,
    },
    While {
        cond: Box<Expression>,
        eval: Box<Expression>,
    },
    Loop(Box<Expression>),
    Continue,
    Break(Option<Box<Expression>>),

    Operator {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    UnaryOperator {
        op: Operator,
        value: Box<Expression>,
    },
    Access {
        left: Box<Expression>,
        right: String,
    },
    Index(Box<Expression>, Vec<Expression>),

    Identifier(String),
    OptionalIdentifier(String),

    Call(Box<Expression>, Vec<Element>),
    Method(Box<Expression>, String),

    Fn(Vec<Element>, Box<Expression>),
    Return(Option<Box<Expression>>),

    Let(String),
    Mut(String),

    Table(Vec<Element>),
}

impl Expression {
    pub fn op_unary_minus(r: Expression) -> Expression {
        Expression::UnaryOperator {
            op: Operator::Minus,
            value: Box::new(r),
        }
    }

    pub fn op_minus(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Minus,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn op_access(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Access,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn op_cat(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Cat,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn op_assign(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Assign,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn op_exponent(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Exponent,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn op_plus(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Plus,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn op_multiply(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Multiply,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn op_divide(l: Expression, r: Expression) -> Expression {
        Expression::Operator {
            op: Operator::Divide,
            left: Box::new(l),
            right: Box::new(r),
        }
    }

    pub fn cond(l: Expression, r: Expression) -> Expression {
        Expression::Conditional {
            cond: Box::new(l),
            eval: Box::new(r),
        }
    }

    pub fn b_true() -> Expression {
        Expression::Bool(true)
    }

    pub fn b_false() -> Expression {
        Expression::Bool(false)
    }

    pub fn err(s: &str) -> Expression {
        Expression::Err(String::from(s))
    }

    pub fn nil(s: &str) -> Expression {
        Expression::Err(String::from(s))
    }

    pub fn l_while(cond: Expression, eval: Expression) -> Expression {
        Expression::While {
            cond: Box::new(cond),
            eval: Box::new(eval),
        }
    }

    pub fn l_loop(eval: Expression) -> Expression {
        Expression::Loop(Box::new(eval))
    }
}
