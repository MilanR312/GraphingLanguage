use chumsky::{input::ValueInput, prelude::*};

use crate::{lexer::Token, spanned::Span};

/// operations with 2 operands (a op b)
#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub enum BinaryOp{
    /// +
    Add,
    /// -
    Subtract,
    /// *
    Multiply,
    /// /
    Divide,
}
impl BinaryOp {
    pub fn add<'src, I: ValueInput<'src, Token = Token, Span = Span>>() -> impl Parser<'src, I, Self> + Clone {
        just(Token::Plus).to(Self::Add)
    }
    pub fn subtract<'src, I: ValueInput<'src, Token = Token, Span = Span>>() -> impl Parser<'src, I, Self> + Clone {
        just(Token::Minus).to(Self::Subtract)
    }
    pub fn multiply<'src, I: ValueInput<'src, Token = Token, Span = Span>>() -> impl Parser<'src, I, Self> + Clone {
        just(Token::Star).to(Self::Multiply)
    }
    pub fn divide<'src, I: ValueInput<'src, Token = Token, Span = Span>>() -> impl Parser<'src, I, Self> + Clone {
        just(Token::Slash).to(Self::Divide)
    }
}