use chumsky::{input::ValueInput, prelude::*};

use crate::{lexer::Token, spanned::{Span, Spanned, SpannedParser}};

use super::{expression::Expression, pattern::Pattern, ty::Type};

#[salsa::tracked(debug)]
pub struct Variable<'db> {
    pub name: Spanned<Pattern<'db>>,
    pub ty: Option<Spanned<Type<'db>>>,
    pub body: Spanned<Expression<'db>>
}
impl<'db> Variable<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src
    {
        let type_annotation = just(Token::DPoint)
            .ignore_then(Type::parser(db).spanned())
            .or_not();
        just(Token::Let)
            .ignore_then(Pattern::parser(db).spanned())
            .then(type_annotation)
            .then_ignore(just(Token::Equals))
            .then(Expression::parser(db).spanned())
            .then_ignore(just(Token::Semicolon))
            .map(|((name, ty), body)| Self::new(db, name, ty, body))
    }
}
#[cfg(test)]
mod tests {
    use crate::{lexer::{lex_source, LexedSource}, stream::Stream, GraphingDatabase, ProgramSource};

    use super::*;

    #[salsa::tracked]
    fn test_compile_variable<'db>(db: &'db dyn salsa::Database, ls: LexedSource<'db>) -> Variable<'db> {
        let tokenstream = ls.tokens(db);
        let stream = Stream::from_iter(tokenstream.to_owned().into_iter());
        Variable::parser(db).parse(stream).unwrap()
    }

    #[test]
    fn variable(){
        let dbs = GraphingDatabase::default();
        let code = "let a = a * 2 ;";
        let code = ProgramSource::new(&dbs, code.to_owned());
        let lexed = lex_source(&dbs, code);
        let _ = test_compile_variable(&dbs, lexed);
    }
    #[test]
    fn pattern_variable(){
        let dbs = GraphingDatabase::default();
        let code = "let (a, b) = a * 2 ;";
        let code = ProgramSource::new(&dbs, code.to_owned());
        let lexed = lex_source(&dbs, code);
        let _ = test_compile_variable(&dbs, lexed);
    }
    #[test]
    fn typed_variable(){
        let dbs = GraphingDatabase::default();
        let code = "let a: u8 = a * 2 ;";
        let code = ProgramSource::new(&dbs, code.to_owned());
        let lexed = lex_source(&dbs, code);
        let _ = test_compile_variable(&dbs, lexed);
    }
}

