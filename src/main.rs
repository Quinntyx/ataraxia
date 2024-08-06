#[cfg(test)]
mod test;

use chumsky::input::Stream;
use chumsky::prelude::*;
use logos::Logos;

use ataraxia::lexer::Token;
use ataraxia::parser::parser;
use ataraxia::interpreter::eval;
use ataraxia::model::object::scope::Scope;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let unwrap = std::env::args().nth(2).unwrap_or_else(|| "".to_owned());

    println!("Testing lexer");
    println!("--- Input string --- ");
    println!("{}", src);
    println!("------- end --------");
    println!("Running lexer");

    let mut tokens = vec![];

    if unwrap == "false" {
        dbg!(Token::lexer(&src).collect::<Vec<_>>());
        println!("Lexer running in fallible mode, exiting testing now...");
        panic!();
    } else {
        tokens.extend(dbg!(Token::lexer(&src)
            .spanned()
            .map(|(t, l)| (t.unwrap(), l))
            .collect::<Vec<_>>()));
    }

    println!("Lexer test completed.");
    println!("Testing parser");

    let eoi = src.len()..src.len();

    let parser_output = dbg!(parser().parse(
        Stream::from_iter(tokens.into_iter().map(|(t, s)| (t, s.into()))).spanned(eoi.into())
    ));

    println!("Parser test completed.");
    println!("Testing interpreter");

    dbg!(eval(parser_output.unwrap(), Scope::new()));

    // println!("{:?}", parser().parse(src));
}
