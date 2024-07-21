// use chumsky::prelude::*;
// use chumsky::input::Stream;
// use logos::Lexer;
//
// use crate::model::expression::Expression as Expr;
// use crate::model::object::primitive::Primitive as P;
// use crate::lexer::Token;
// use crate::lexer::TokenKind;
//
// type AxParser<'a> = impl Parser<'a, Stream<'a, Token<'a>>, Expr>;
//
// type AtaraxiaParser<'a, T: Iterator> = impl Parser<'a, Stream<T>, Expr>;
//
// fn parse_expr<'a>(block: AxParser<'a>, expr: AxParser<'a>) -> AxParser<'a> {
//     let int = just(Token::Add).map(|t: Token| {
//         
//     });
//
//     int
// }
//
// fn parse_block<'a>(block: AxParser<'a, T>) -> AxParser<'a> {
//     let expr = recursive(|expr| parse_expr(block, expr));
//     expr //TODO
// }
//
// pub fn parser<'a>() -> AxParser<'a> {
//     recursive(parse_block)
// }
//
// pub fn parser_old() {
//         //     
//         //     let int = text::int(10)
//         //         .map(|s: String| Expr::Literal(P::Integer(s.parse().unwrap())))
//         //         .padded();
//         //
//         //     let atom = int
//         //         .or(expr.clone()
//         //             .delimited_by(just('(').padded(), just(')').padded())
//         //             .or(block.delimited_by(just('{').padded(), just('}').padded())))
//         //         .padded();
//         //
//         //     let op = |c| just(c).padded();
//         //     let kwd = |c| just(c).padded();
//         //
//         //     atom
//         // });
//         let expr = recursive(|expr| {
//             let int = text::int(10)
//                 .map(|s: String| Expr::Literal(P::Integer(s.parse().unwrap())))
//                 .padded();
//
//             let atom = int
//                 .or(expr.clone()
//                     .delimited_by(just('(').padded(), just(')').padded())
//                     .or(block.delimited_by(just('{').padded(), just('}').padded())))
//                 .padded();
//
//             let op = |c| just(c).padded();
//             let kwd = |c| just(c).padded();
//
//             let unary = op('-')
//                 .repeated()
//                 .then(atom)
//                 .foldr(|_op, rhs| Expr::op_unary_minus(rhs));
//
//             let product = unary
//                 .clone()
//                 .then(
//                     op('*')
//                         .to(Expr::op_multiply as fn(_, _) -> _)
//                         .or(op('/').to(Expr::op_divide as fn(_, _) -> _))
//                         .then(unary)
//                         .repeated(),
//                 )
//                 .foldl(|lhs, (op, rhs)| op(lhs, rhs));
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
//                 .foldl(|lhs, (op, rhs)| op(lhs, rhs));
//
//             let cond = kwd("if")
//                 .then(expr.clone())
//                 .then(kwd("do"))
//                 .then(expr.clone())
//                 .map(|(l, r)| Expr::cond as fn(_, _) -> _);
//
//             let ifelse = cond.clone()
//                 .then(kwd("else").then(cond.clone()))
//                 .repeated()
//                 .then(kwd("else do")).then(expr.clone())
//                 .repeated()
//                 .at_most(1)
//                 .foldl(|lhs, (op, rhs)| {
//                     lhs.cases.push(rhs);
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
//                             cases.push(rhs);
//                             Expr::If {
//                                 cases
//                             }
//                         },
//                         _ => unreachable!("If statement parsing crash, this is a bug")
//                     }
//                 });
//
//             ifelse
//         });
//         expr // TODO
// }
