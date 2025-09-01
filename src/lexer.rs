use logos::Logos;
use ordered_float::OrderedFloat;

use crate::{spanned::Span, ProgramSource};
use crate::stream::Stream;


#[salsa::tracked(debug)]
pub struct LexedSource<'db> {
    #[returns(ref)]
    pub tokens: Vec<(Token, Span)>
}


#[derive(Logos, Debug, PartialEq, Clone, Hash, Eq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    // ===== Literals =====
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Int(i64),

    #[regex(r"[0-9]+\.[0-9]+([eE][+-]?[0-9]+)?", |lex| OrderedFloat(lex.slice().parse::<f64>().unwrap()))]
    Float(OrderedFloat<f64>),

    #[token("true")]
    True,
    #[token("false")]
    False,

    // ===== Identifiers =====
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_owned())]
    Identifier(String),

    // Wildcard pattern
    #[token("_", priority = 10)]
    Wildcard,

    // ===== Operators =====
    #[token("=")]
    Equals,
    #[token("&")]
    Ampersand,
    #[token("|")]
    Pipe,
    #[token(">")]
    GreaterThan,
    #[token("<")]
    LessThan,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("!")]
    Bang,

    // ===== Delimiters / punctuation =====
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    DPoint,

    // ===== Keywords =====
    #[token("let")]
    Let,
    #[token("fn")]
    Fn,
    #[token("if")]
    If,
    #[token("else")]
    Else,

    // ===== Comments (optional) =====
    #[regex(r"//[^\n]*", logos::skip)]
    Comment,

    // ===== Error =====
    Error,
}

#[salsa::tracked]
pub fn lex_source(db: &dyn salsa::Database, code: ProgramSource) -> LexedSource<'_> {
    let source = code.raw_text(db);
    println!("running lexer");
    let tokenstream = Token::lexer(source).spanned()
        .map(|(tok, span)| {
            let span = Span::new(span.start, span.end);
            let token = tok.unwrap_or(Token::Error);
            (token, span)
        });
    LexedSource::new(db, tokenstream.collect())
}

#[cfg(test)]
pub fn lex_to_stream(source: &str) -> Stream<impl Iterator<Item = (Token, Span)>>
{

    let tokenstream = Token::lexer(source).spanned()
        .map(|(tok, span)| {
            let span = Span::new(span.start, span.end);
            let token = tok.unwrap_or(Token::Error);
            (token, span)
        });
    Stream::from_iter(tokenstream)
}

