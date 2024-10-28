use chumsky::input::ValueInput;
use chumsky::prelude::*;

#[allow(unused_imports)] // FIXME: Rich Errors
use chumsky::error::{Rich, Simple};
use chumsky::extra::Err;
use chumsky::span::SimpleSpan;

use crate::lexer::Token;
use crate::model::expression::Element;
use crate::model::expression::Expression as Expr;


pub fn parser<'a, T>() -> impl Parser<'a, T, Expr, Err<Rich<'a, Token>>> + Clone
// FIXME: Error handling
where
    T: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let mut expr = Recursive::declare();
    let mut block = Recursive::declare();
    let mut table = Recursive::declare();

    let int = any()
        .filter(|t| matches!(t, Token::Integer(_)))
        .map(|t| match t {
            Token::Integer(v) => Expr::Integer(v),
            _ => unreachable!("Attempted to parse integer from other token type"),
        })
        .labelled("integer")
        .boxed();

    let ident = any()
        .filter(|t| matches!(t, Token::Ident(_)))
        .map(|t| match t {
            Token::Ident(v) => Expr::Identifier(v),
            _ => unreachable!("Attempted to parse identifier from other token type"),
        })
        .labelled("identifier")
        .boxed();

    let ident_as_str = any()
        .filter(|t| matches!(t, Token::Ident(_)))
        .map(|t| match t {
            Token::Ident(v) => v,
            _ => unreachable!("Attempted to parse identifier from other token type"),
        })
        .labelled("identifier")
        .boxed();

    let kvset = ident
        .clone()
        .then_ignore(just(Token::Assign))
        .then(expr.clone())
        .map(Element::KV)
        .labelled("key-value pair")
        .or(expr.clone().map(Element::V))
        .labelled("array value")
        .separated_by(just(Token::ItemSep).labelled("comma `,`"))
        .allow_trailing()
        .collect::<Vec<_>>()
        .labelled("table-formatted fields")
        .boxed();

    let r#let = just(Token::Let)
        .labelled("let")
        .ignore_then(ident_as_str.clone())
        .then_ignore(just(Token::Assign))
        .then(expr.clone())
        .map(|(i, e)| Expr::Let(i, Box::new(e)))
        .boxed();

    let r#mut = just(Token::Mut)
        .labelled("mut")
        .ignore_then(ident_as_str.clone())
        .then_ignore(just(Token::Assign))
        .then(expr.clone())
        .map(|(i, e)| Expr::Mut(i, Box::new(e)))
        .boxed();

    let r#continue = just(Token::Continue)
        .to(Expr::Continue)
        .labelled("continue")
        .boxed();

    let r#break = just(Token::Break)
        .labelled("break")
        .ignore_then(
            expr.clone()
                .map(|i| Box::new(i))
                .labelled("break body (expression)")
                .or_not(),
        )
        .map(|e| Expr::Break(e))
        .boxed();

    let r#return = just(Token::Return)
        .labelled("return")
        .ignore_then(
            expr.clone()
                .map(|i| Box::new(i))
                .labelled("return body (expression)")
                .or_not(),
        )
        .map(|e| Expr::Return(e))
        .boxed();

    let r#fn = just(Token::Function)
        .labelled("fn")
        .ignore_then(
            kvset
                .clone()
                .delimited_by(
                    just(Token::OpenParen).labelled("opening parenthesis"),
                    just(Token::CloseParen).labelled("closing parenthesis"),
                )
                .labelled("function argument list"),
        )
        .then(expr.clone().labelled("function body (expression)"))
        .map(|(kv, e)| Expr::Fn(kv, Box::new(e)))
        .boxed();

    let r#while = just(Token::While)
        .labelled("while")
        .ignore_then(expr.clone())
        .then_ignore(just(Token::Do).labelled("do"))
        .then(expr.clone().labelled("while body (expression)"))
        .map(|(c, e)| Expr::l_while(c, e))
        .boxed();

    let r#loop = just(Token::Loop)
        .labelled("loop")
        .ignore_then(expr.clone().labelled("loop body (expression)"))
        .map(|e| Expr::l_loop(e))
        .boxed();

    let ifelse = just(Token::If)
        .labelled("if")
        .ignore_then(expr.clone().labelled("if condition (expression)"))
        .then_ignore(just(Token::Do).labelled("do"))
        .then(expr.clone().labelled("if body (expression)"))
        .map(|(c, e)| Expr::cond(c, e))
        .labelled("if expression")
        .separated_by(just(Token::Else).labelled("else"))
        .at_least(1)
        .collect::<Vec<_>>()
        .then(
            just(Token::Else)
                .labelled("else")
                .ignore_then(just(Token::Do).labelled("do"))
                .ignore_then(expr.clone().labelled("else body (expression)"))
                .or_not()
                .map(|e| {
                    Expr::cond(
                        Expr::b_true(),
                        e.unwrap_or(Expr::nil("fell through if, no specified else")),
                    )
                }),
        )
        .map(|(mut v, e)| {
            v.push(e);
            Expr::If(v)
        })
        .boxed();

        

    let string = any()
        .filter(|t| matches!(t, Token::String(_)))
        .map(|t| match t {
            Token::String(v) => Expr::String(v),
            _ => unreachable!("Attempted to parse string from other token type"),
        })
        .labelled("string")
        .boxed();

    let bool_as_bool = any()
        .filter(|t| matches!(t, Token::Bool(_)))
        .map(|t| match t {
            Token::Bool(b) => b,
            _ => unreachable!("Attempted to parse bool from other token type"),
        })
        .labelled("boolean")
        .boxed();

    let bool = bool_as_bool
        .clone()
        .map(|b| Expr::Bool(b))
        .labelled("boolean")
        .boxed();

    let float_as_float = any()
        .filter(|t| matches!(t, Token::Frac(_)))
        .map(|t| match t {
            Token::Frac(v) => v,
            _ => unreachable!("Attempted to parse float from other token type"),
        })
        .labelled("fraction")
        .boxed();

    let float = float_as_float
        .clone()
        .map(|t| Expr::Frac(t))
        .labelled("fraction")
        .boxed();

    // let range_nol = just(Token::Range)
    //     .labelled("range `..`")
    //     .ignore_then(expr.clone().labelled("range end (expression)"))
    //     .or(just(Token::RangeInclusive)
    //         .labelled("inclusive range `..=`")
    //         .ignore_then(
    //             expr.clone()
    //                 .labelled("inclusive range end (expression)")
    //                 .map(|i| Expr::op_plus(i, Expr::Integer(1))),
    //         ))
    //     .map(|i| Box::new(i))
    //     .or_not()
    //     .map(|r| Expr::Range(None, r));

    let atom = choice((
        int,
        string,
        bool,
        float,
        // range_nol,
        expr.clone()
            .delimited_by(just(Token::OpenParen), just(Token::CloseParen)),
        block
            .clone()
            .delimited_by(just(Token::OpenBlock), just(Token::CloseBlock)),
        table
            .clone()
            .delimited_by(just(Token::OpenTable), just(Token::CloseTable)),
        ident,
        r#let,
        r#mut,
        r#continue,
        r#break,
        ifelse,
        r#return,
        r#while,
        r#loop,
        r#fn,
    ));

    // FIXME?: RangeInclusive and Range have the same semantics here
    // let range_nor = atom
    //     .clone()
    //     .then_ignore(just(Token::Range).labelled("range-op-nor"))
    //     .or(just(Token::RangeInclusive))//.labelled("inclusive-range-op-nor"))
    //     .map(|i| Expr::Range(Some(Box::new(i)), None))
    //     .or(atom.clone())
    //     .boxed();
    //
    // let range = range_nor
    //     .clone()
    //     .foldl(
    //         just(Token::Range)
    //             .ignore_then(expr.clone())
    //             .or(just(Token::RangeInclusive)
    //                 .ignore_then(expr.clone().map(|i| Expr::op_plus(i, Expr::Integer(1)))))
    //             .map(|i| Box::new(i))
    //             .repeated(),
    //         |l, r| Expr::Range(Some(Box::new(l)), Some(r)),
    //     )
    //     .boxed();

    let method = atom
        .clone()
        .foldl(
            just(Token::Method)
                .ignore_then(ident_as_str.clone())
                .repeated(),
            |e, i| Expr::Method(Box::new(e), i),
        )
        .boxed();

    let index = method
        .clone()
        .foldl(
            expr.clone()
                .separated_by(just(Token::ItemSep))
                .at_least(1)
                .collect::<Vec<_>>()
                .delimited_by(just(Token::OpenTable), just(Token::CloseTable))
                .repeated(),
            |f, a| Expr::Index(Box::new(f), a),
        )
        .boxed();

    let call = index
        .clone()
        .foldl(
            kvset
                .clone()
                .delimited_by(just(Token::OpenParen), just(Token::CloseParen))
                .repeated(),
            |f, a| Expr::Call(Box::new(f), a),
        )
        .boxed();

    let cat = call
        .clone()
        .foldl(
            just(Token::Cat).ignore_then(expr.clone()).repeated(),
            |a, b| Expr::op_cat(a, b),
        )
        .boxed();

    let unary = just(Token::Sub)
        .repeated()
        .foldr(cat.clone(), |_op, rhs| Expr::op_unary_minus(rhs))
        .boxed();

    let access = unary
        .clone()
        .foldl(
            just(Token::Access).ignore_then(unary).repeated(),
            |lhs, rhs| Expr::op_access(lhs, rhs),
        )
        .boxed();

    let assign = access
        .clone()
        .foldl(
            just(Token::Assign).ignore_then(access).repeated(),
            |lhs, rhs| Expr::op_assign(lhs, rhs),
        )
        .boxed();

    let power = assign
        .clone()
        .foldl(
            just(Token::Pow).ignore_then(assign).repeated(),
            |lhs, rhs| Expr::op_exponent(lhs, rhs),
        )
        .boxed();

    let product = power
        .clone()
        .foldl(
            just(Token::Mul)
                .to(Expr::op_multiply as fn(_, _) -> _)
                .or(just(Token::Div).to(Expr::op_divide as fn(_, _) -> _))
                .then(power.clone())
                .repeated(),
            |lhs, (op, rhs)| op(lhs, rhs),
        )
        .boxed();

    let sum = product
        .clone()
        .foldl(
            just(Token::Add)
                .to(Expr::op_plus as fn(_, _) -> _)
                .or(just(Token::Sub).to(Expr::op_minus as fn(_, _) -> _))
                .then(product.clone())
                .repeated(),
            |lhs, (op, rhs)| op(lhs, rhs),
        )
        .boxed();

    expr.define(sum.labelled("expr"));

    block
        .define(
            expr.clone()
                .then_ignore(just(Token::Semicolon))
                .repeated()
                .collect::<Vec<_>>()
                .then(expr.clone().or_not())
                .map(|(exps, rtn)| Expr::Block(exps, Box::new(rtn.unwrap_or(Expr::Noop))))
                .labelled("block"),
        );

    table.define(kvset.clone().map(|v| Expr::Table(v)).labelled("table"));

    block
}
//             let product = unary
//                 .clone()
//                 .then(
//                     op('*')
//                         .to(Expr::op_multiply as fn(_, _) -> _)
//                         .or(op('/').to(Expr::op_divide as fn(_, _) -> _))
//                         .then(unary)
//                         .repeated(),
//                 )
//                 .foldl(|lhs, (op, rhs)| op(lhs, rhs))
//
//             let sum = product
//                 .clone()
//                 .then(
//                     op('+')
//                         .to(Expr::op_multiply as fn(_, _) -> _)
//                         .or(op('-').to(Expr::op_divide as fn(_, _) -> _))
//                         .then(product)
//                         .repeated(),
//                 )
//                 .foldl(|lhs, (op, rhs)| op(lhs, rhs))
//
//             let cond = kwd("if")
//                 .then(expr.clone())
//                 .then(kwd("do"))
//                 .then(expr.clone())
//                 .map(|(l, r)| Expr::cond as fn(_, _) -> _)
//
//             let ifelse = cond.clone()
//                 .then(kwd("else").then(cond.clone()))
//                 .repeated()
//                 .then(kwd("else do")).then(expr.clone())
//                 .repeated()
//                 .at_most(1)
//                 .foldl(|lhs, (op, rhs)| {
//                     lhs.cases.push(rhs)
//                     lhs
//                 })
//
//
//                 .then(kwd("else")
//                     .then(cond)
//                     .repeated()
//                     .then(kwd("else"))
//                     .repeated()
//                     .at_most(1))
//                 .foldl(|lhs, (op, rhs)| {
//                     match lhs {
//                         Expr::Conditional { .. } => Expr::If {
//                             cases: vec![lhs, rhs],
//                         },
//                         Expr::If { cases } => {
//                             cases.push(rhs)
//                             Expr::If {
//                                 cases
//                             }
//                         },
//                         _ => unreachable!("If statement parsing crash, this is a bug")
//                     }
//                 })
//
//             ifelse
//         })
//         expr // TODO
// }
