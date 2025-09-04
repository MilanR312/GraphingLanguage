use chumsky::{extra::ParserExtra, input::{IterInput, ValueInput}, pratt::{self, infix, left}, prelude::*};
use logos::Logos;
use ordered_float::OrderedFloat;


mod stream;
mod spanned;
mod lexer;
mod parser;
mod ids;
mod symbols;
use ids::*;
use parser::*;
use lexer::*;
use salsa::Setter;
use stream::Stream;
use spanned::Span;
use symbols::{create_scope_parent_table, create_symbol_table};

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


#[salsa::accumulator]
#[derive(Debug)]
pub struct ParseError {
    pub start: usize,
    pub end: usize,
    pub message: String,
}





fn main(){
    return;
    let mut db = GraphingDatabase::default();
    println!("run 1");
    let src = r#"
    let x = 1;
    let a = 2;
    fn foo(x) = {
        let a = 2 * x;
        a
    };
    let y = {
        fn bar(x) = 2 * x;
        2 * 3 //bar(x)
    };
    "#;
    let src = ProgramSource::new(&db, src.to_owned());
    let lexed = lex_source(&db, src);
    let compiled = compile_tokenstream(&db, lexed);
    
    let parent_map = create_scope_parent_table(&db, compiled);
    let symbol_table = create_symbol_table(&db, compiled);
    let table = symbol_table.items(&db);
    // simulate a change to the code
    let src2 = r#"
    let x = 1;
    let a = 2;
    fn foo(x) = {
        let a = 2 * x;
        a
    };
    let y = {
        fn bar(x) = 2 * x;
        2 * 3 //bar(x)
    };
    "#;
    let a = src.set_raw_text(&mut db).to(src2.to_owned());
    println!("run 2");

    let lexed = lex_source(&db, src);
    let compiled = compile_tokenstream(&db, lexed);
    
    let parent_map = create_scope_parent_table(&db, compiled);
    let symbol_table = create_symbol_table(&db, compiled);
    let table = symbol_table.items(&db);

    //let compiled = compile_tokenstream(&db, lexed);
    //let compiled = compiled.statements(&db);


    //println!("{compiled:#?}");
}