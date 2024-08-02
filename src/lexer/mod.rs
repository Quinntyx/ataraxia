use crate::model::object::fraction::Fraction;

use logos::Logos;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum Token {
    // Keywords
    #[token("fn", priority = 100)]
    Function,
    #[token("return", priority = 100)]
    Return,
    #[token("do", priority = 100)]
    Do,
    #[token("for", priority = 100)]
    For,
    #[token("loop", priority = 100)]
    Loop,
    #[token("while", priority = 100)]
    While,
    #[token("break", priority = 100)]
    Break,
    #[token("continue", priority = 100)]
    Continue,
    #[token("let", priority = 100)]
    Let,
    #[token("mut", priority = 100)]
    Mut,
    #[token("in", priority = 100)]
    In,
    #[token("if", priority = 100)]
    If,
    #[token("else", priority = 100)]
    Else,
    #[token("err", priority = 100)]
    Err,
    #[token("nil", priority = 100)]
    Nil,

    // Operators
    #[token("+", priority = 100)]
    Add,
    #[token("-", priority = 100)]
    Sub,
    #[token("*", priority = 100)]
    Mul,
    #[token("**", priority = 150)]
    Pow,
    #[token("/", priority = 100)]
    Div,
    #[token("~", priority = 100)]
    Cat,
    #[token("..", priority = 250)]
    Range,
    #[token("..=", priority = 300)]
    RangeInclusive,
    #[token("==", priority = 100)]
    Eq,
    #[token("!=", priority = 100)]
    Neq,
    #[token("=", priority = 100)]
    Assign,
    #[token("&", priority = 100)]
    And,
    #[token("|", priority = 100)]
    Or,
    #[token(".", priority = 100)]
    Access,
    #[token(":", priority = 100)]
    Method,

    // Braces
    #[token("(", priority = 100)]
    OpenParen,
    #[token(")", priority = 100)]
    CloseParen,
    #[token("{", priority = 100)]
    OpenBlock,
    #[token("}", priority = 100)]
    CloseBlock,
    #[token("[", priority = 100)]
    OpenTable,
    #[token("]", priority = 100)]
    CloseTable,

    // Primitives
    #[regex(r#""(?:\\"|[^"])*""#, |lex| lex.slice().trim_matches('\"').to_owned(), priority=50)]
    #[regex(r#"'(?:\\'|[^'])*'"#, |lex| lex.slice().trim_matches('\'').to_owned(), priority=50)]
    String(String),
    #[token("true", |_| true, priority=100)]
    #[token("false", |_| false, priority=100)]
    Bool(bool),
    #[regex(r"(?:\d)*", |lex| lex.slice().parse::<i64>().unwrap(), priority=150)]
    Integer(i64),
    // #[regex(r"\d+\.\d+|\d+\.|\.\d+", |lex| dbg!(lex.slice()).parse::<f64>().unwrap(), priority=200)]
    #[regex(r"\d+\.\d+", |lex| {
        let mut iter = lex.slice().split('.');
        let int = iter.next().unwrap();
        let remainder = iter.next().unwrap();
        Fraction {
            numerator: format!("{}{}", int, remainder).parse::<i64>().unwrap().into(),
            denominator: 10u32.pow(remainder.len() as u32).into(),
        }
    }, priority=200)]
    Frac(Fraction),

    // Comments
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", |lex| lex.slice().split_at(2).1.trim().to_owned())]
    InlineComment(String),
    #[regex(r"//.*\n?", |lex| lex.slice().split_at(2).1.trim().to_owned())]
    LineComment(String),

    // Identifiers
    #[token(";", priority = 100)]
    Semicolon,
    #[token(",", priority = 100)]
    ItemSep,
    #[regex(r#"(?:[^\t\r\n\f+\- @&#$\^|%()?,\.*!;'"\[\]\{\}0123456789:])(?:[^\t\r\n\f+\- @&#$\^|%()?,*!;'"\[\]\.\{\}:])*"#, |lex| lex.slice().to_owned())]
    Ident(String),
}
