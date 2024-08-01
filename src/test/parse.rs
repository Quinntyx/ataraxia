use chumsky::input::Stream;
use chumsky::prelude::*;
use logos::Logos;

use ataraxia::lexer::Token;
use ataraxia::parser::parser;

fn lp(s: &str) {
    dbg!(parser()
        .parse(Stream::from_iter(
            Token::lexer(s)
                .map(|i| i.unwrap())
                .collect::<Vec<_>>()
                .into_iter(),
        ))
        .unwrap());
}

#[test]
fn r#while() {
    lp("while a do b")
}

#[test]
fn r#loop() {
    lp("loop a()")
}

#[test]
fn r#continue() {
    lp("continue")
}

#[test]
fn r#break() {
    lp("break")
}

#[test]
fn break_ret() {
    lp("break a")
}

#[test]
fn r#return() {
    lp("return")
}

#[test]
fn return_ret() {
    lp("return a")
}

#[test]
fn float() {
    lp("1.1")
}

#[test]
fn float_no_leading() {
    lp(".1")
}

#[test]
fn float_no_trailing() {
    lp("1.")
}

#[test]
fn r#true() {
    lp("true")
}

#[test]
fn r#false() {
    lp("false")
}

#[test]
fn r#fn() {
    lp("fn() a")
}

#[test]
fn fn_semi() {
    lp("fn() a;")
}

#[test]
fn fn_arg() {
    lp("fn(a) a")
}

#[test]
fn fn_arg_semi() {
    lp("fn(a) a;")
}

#[test]
fn fn_multiarg() {
    lp("fn(a, b) a")
}

#[test]
fn fn_multiarg_semi() {
    lp("fn(a, b) a;")
}

#[test]
fn fn_kv() {
    lp("fn(a = b) a")
}

#[test]
fn fn_kv_semi() {
    lp("fn(a = b) a;")
}

#[test]
fn fn_mixed() {
    lp("fn(a, b = c) a")
}

#[test]
fn fn_mixed_semi() {
    lp("fn(a, b = c) a;")
}

#[test]
fn fn_curried() {
    lp("fn(x) fn(y) x + y")
}

#[test]
fn fn_curried_semi() {
    lp("fn(x) fn(y) x + y;")
}

#[test]
fn fn_block() {
    lp("fn(x) { a(); x }")
}

#[test]
fn range() {
    lp("1..3")
}

#[test]
fn range_open_front() {
    lp("..3")
}

#[test]
fn range_open_back() {
    lp("3..")
}

#[test]
fn range_all() {
    lp("..")
}

#[test]
fn index() {
    lp("a[3]")
}

#[test]
fn index_multi() {
    lp("a[1, 1]")
}

#[test]
fn index_expr() {
    lp("a[1 + 3]")
}

#[test]
fn index_expr_multi() {
    lp("a[1 + 2, 3 + 4]")
}

#[test]
fn cat() {
    lp("a ~ b")
}

#[test]
fn cat_multi() {
    lp("a ~ b ~ c")
}

#[test]
fn construct_array_table() {
    lp("[1]")
}

#[test]
fn construct_array_table_multiarg() {
    lp("[ 1, 2 ]")
}

#[test]
fn construct_array_table_kv() {
    lp("[ a = 1 ]")
}

#[test]
fn construct_array_table_multiarg_kv() {
    lp("[ a = 1, b = 2 ]")
}

#[test]
fn construct_array_table_mixed() {
    lp("[ a = 1, 2 ]")
}

#[test]
fn r#let() {
    lp("let x")
}

#[test]
fn let_semi() {
    lp("let x;")
}

#[test]
fn let_assign() {
    lp("let x = 1")
}

#[test]
fn let_assign_semi() {
    lp("let x = 1;")
}

#[test]
fn let_assign_table() {
    lp("let x = [ 1 ]")
}

#[test]
fn let_assign_table_semi() {
    lp("let x = [ 1 ];")
}

#[test]
fn r#mut() {
    lp("mut x")
}

#[test]
fn mut_semi() {
    lp("mut x;")
}

#[test]
fn mut_assign() {
    lp("mut x = 1")
}

#[test]
fn mut_assign_semi() {
    lp("mut x = 1;")
}

#[test]
fn mut_assign_table() {
    lp("mut x = [ 1 ]")
}

#[test]
fn mut_assign_table_semi() {
    lp("mut x = [ 1 ];")
}

#[test]
fn method() {
    lp("x:a")
}

#[test]
fn method_semi() {
    lp("x:a;")
}

#[test]
fn method_call() {
    lp("x:a()")
}

#[test]
fn call() {
    lp("x()")
}

#[test]
fn call_semi() {
    lp("x();")
}

#[test]
fn call_args() {
    lp("x(1)")
}

#[test]
fn call_multiargs() {
    lp("x(1, 2)")
}

#[test]
fn call_kv() {
    lp("x(a = 1)")
}

#[test]
fn call_kv_multiargs() {
    lp("x(a = 1, b = 2)")
}

#[test]
fn call_mixed() {
    lp("x(1, a = 2)")
}

#[test]
fn int() {
    lp("5")
}

#[test]
fn dquote_str() {
    lp(r#""test""#)
}

#[test]
fn squote_str() {
    lp("'test'")
}

#[test]
fn mul() {
    lp("1 * 2")
}

#[test]
fn div() {
    lp("1 / 2");
}

#[test]
fn add() {
    lp("1 + 2");
}

#[test]
fn sub() {
    lp("1 - 2");
}

#[test]
fn pow() {
    lp("1 ** 2")
}

#[test]
fn neg() {
    lp("-3")
}

#[test]
fn neg_multi() {
    lp("--3")
}

#[test]
fn math_expr() {
    lp("1 + 2 / 3 * 5")
}

#[test]
fn math_expr_parens() {
    lp("(1 + 2) / (3 * 4)")
}

#[test]
fn r#if() {
    lp("if 5 do 5")
}

#[test]
fn if_else() {
    lp("if 5 do 5 else do 5")
}

#[test]
fn if_elseif() {
    lp("if 5 do 5 else if 6 do 5")
}

#[test]
fn if_elseif_else() {
    lp("if 5 do 5 else if 6 do 5 else do 5")
}
