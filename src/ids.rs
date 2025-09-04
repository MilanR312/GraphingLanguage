use std::sync::atomic::{AtomicU64, Ordering};

use chumsky::{input::ValueInput, prelude::*};
use crate::{lexer::Token, spanned::Span};


#[salsa::interned(debug)]
pub struct VariableId<'db>{
    #[returns(ref)]
    pub text: String,
}
impl<'db> VariableId<'db> {
    pub fn parser<'src, I: ValueInput<'src, Token = Token, Span = Span>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone {
        select! { Token::Identifier(x) => x }
            .map(|x| Self::new(db, x))
    }
}

#[salsa::interned(debug)]
pub struct FunctionId<'db> {
    #[returns(ref)]
    pub text: String,
}
impl<'db> FunctionId<'db> {
    pub fn parser<'src, I: ValueInput<'src, Token = Token, Span = Span>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone {
        select! { Token::Identifier(x) => x }
            .map(|x| Self::new(db, x))
    }
}

#[salsa::interned(debug)]
pub struct TypeId<'db>{
    #[returns(ref)]
    pub text: String,
}
impl<'db> TypeId<'db> {
    pub fn parser<'src, I: ValueInput<'src, Token = Token, Span = Span>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone {
        select! {Token::Identifier(x) => x}
            .map(|x| Self::new(db, x))
    }
}
#[salsa::interned(debug)]
pub struct BlockId<'db> {
    #[returns(ref)]
    pub id: u64
}

