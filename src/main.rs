use chumsky::{extra::ParserExtra, input::{IterInput, ValueInput}, pratt::{self, infix, left}, prelude::*};
use logos::Logos;
use ordered_float::OrderedFloat;


mod stream;
mod spanned;
mod lexer;
mod parser;
mod ids;
use ids::*;
use parser::*;
use lexer::*;
use stream::Stream;
use spanned::Span;

#[salsa::db]
#[derive(Clone, Default)]
struct GraphingDatabase {
    storage: salsa::Storage<Self>
}
#[salsa::db]
impl salsa::Database for GraphingDatabase{}


#[salsa::input(debug)]
pub struct ProgramSource {
    #[returns(ref)]
    raw_text: String
}





/*


/// TODO: types

#[salsa::tracked(debug)]
pub struct Program<'db> {
    #[tracked]
    #[returns(ref)]
    pub statements: Vec<Statement<'db>>
}
impl<'db> Program<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src
    {
        Statement::parser(db).repeated()
            .collect()
            .map(|x| Self::new(db, x))
    }
}

#[salsa::tracked]
pub fn compile_tokenstream<'db>(db: &'db dyn salsa::Database, ls: LexedSource<'db>) -> Program<'db> {
    let parser = Program::parser(db);
    let tokenstream = ls.tokens(db);
    let tokenstream = Stream::from_iter(tokenstream.to_owned());
    let parsed = parser.parse(tokenstream).unwrap();
    parsed
}





*/






#[salsa::accumulator]
#[derive(Debug)]
pub struct ParseError {
    pub start: usize,
    pub end: usize,
    pub message: String,
}


fn main(){
    let db = GraphingDatabase::default();
    let src = r#"
    let x = 1;
    let a = 2;
    let (a,b) = (1,2);
    fn foo(x) = x;
    let x = 2;
    "#;
    let src = ProgramSource::new(&db, src.to_owned());
    let lexed = lex_source(&db, src);
    //let compiled = compile_tokenstream(&db, lexed);
    //let compiled = compiled.statements(&db);


    //println!("{compiled:#?}");
}