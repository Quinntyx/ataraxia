use logos::Logos;

#[derive(Logos, Clone, Debug)]
#[logos(skip r"[ \t\r\n\f]+")]
pub enum TokenKind {
    // Keywords
    #[token("fn", priority=100)]
    Function,
    #[token("return", priority=100)]
    Return,
    #[token("do", priority=100)]
    Do,
    #[token("for", priority=100)]
    For,
    #[token("while", priority=100)]
    While,
    #[token("break", priority=100)]
    Break,
    #[token("continue", priority=100)]
    Continue,
    #[token("let", priority=100)]
    Let,
    #[token("var", priority=100)]
    Var,
    #[token("in", priority=100)]
    In,


    // Operators
    #[token("+", priority=100)]
    Add,
    #[token("-", priority=100)]
    Sub,
    #[token("*", priority=100)]
    Mul,
    #[token("/", priority=100)]
    Div,
    #[token("..", priority=100)]
    Cat,
    #[token("==", priority=100)]
    Eq, 
    #[token("!=", priority=100)]
    Neq,
    #[token("=", priority=100)]
    Assign,
    #[token("&", priority=100)]
    And,
    #[token("|", priority=100)]
    Or,
    #[token(".", priority=100)]
    Access,
    #[token(":", priority=100)]
    Method,

    // Braces
    #[token("(", priority=100)]
    OpenParen,
    #[token(")", priority=100)]
    CloseParen,
    #[token("{", priority=100)]
    OpenBlock,
    #[token("}", priority=100)]
    CloseBlock,
    #[token("[", priority=100)]
    OpenTable,
    #[token("]", priority=100)]
    CloseTable,
    
    // Primitives
    #[regex("\"(?:[^\"\n]|\\\")*\"", |lex| lex.slice().to_owned(), priority=50)]
    #[regex("\'(?:[^'\n]|\\\')*\'", |lex| lex.slice().to_owned(), priority=50)]
    String(String),
    #[token("true", |_| true, priority=100)]
    #[token("false", |_| false, priority=100)]
    Bool(bool),
    #[regex(r"(?:\d)*", |lex| lex.slice().parse::<i64>().unwrap(), priority=150)]
    Integer(i64),
    #[regex(r"\d*\.\d*", |lex| lex.slice().parse::<f64>().unwrap(), priority=200)]
    Float(f64),

    // Comments
    #[regex(r"/\*([^*]|\*+[^*/])*\*+/", |lex| lex.slice().to_owned())]
    InlineComment(String),
    #[regex(r"//.*\n?", |lex| lex.slice().to_owned())]
    LineComment(String),

    // Identifiers
    #[token(";", priority=100)]
    Semicolon,
    #[regex(r"\S*", |lex| lex.slice().to_owned())]
    Ident(String),
}
