use chumsky::prelude::*;
use logos::Logos;

// use ataraxia::parser::parser;
use ataraxia::lexer::TokenKind;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("Testing lexer");
    println!("--- Input string --- ");
    println!("{}", src);
    println!("------- end --------");
    println!("Running lexer");

    dbg!(TokenKind::lexer(&src).map(|i| i.unwrap()).collect::<Vec<_>>());

    // println!("{:?}", parser().parse(src));
}
