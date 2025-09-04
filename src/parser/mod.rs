use chumsky::{input::ValueInput, prelude::*};

use crate::{lexer::{LexedSource, Token}, spanned::Span, stream::Stream};


pub mod literal;
pub mod pattern;
pub mod ty;
pub mod operations;
pub mod expression;
pub mod variable;
pub mod function;
pub mod statement;
pub mod block;



#[salsa::tracked(debug)]
pub struct Program<'db> {
    #[tracked]
    #[returns(ref)]
    pub statements: Vec<statement::Statement<'db>>
}
impl<'db> Program<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src
    {
        statement::Statement::parser(db)
            .repeated()
            .collect::<Vec<_>>()
            .map(|x| Self::new(db, x))
    }
}

#[salsa::tracked]
pub fn compile_tokenstream<'db>(db: &'db dyn salsa::Database, tokenstream: LexedSource<'db>) -> Program<'db> {
    println!("compiling");
    let tokenstream = tokenstream.tokens(db);
    let stream = Stream::from_iter(tokenstream.to_owned().into_iter());
    Program::parser(db).parse(stream).unwrap()
}