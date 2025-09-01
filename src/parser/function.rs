use chumsky::{input::ValueInput, prelude::*};

use crate::{ids::FunctionId, lexer::Token, spanned::{Span, Spanned, SpannedParser}};

use super::{expression::Expression, pattern::Pattern, ty::Type};

#[salsa::tracked(debug)]
pub struct Function<'db> {
    pub name: Spanned<FunctionId<'db>>,
    pub args: Vec<(Spanned<Pattern<'db>>, Option<Spanned<Type<'db>>>)>,
    pub return_type: Option<Spanned<Type<'db>>>,
    pub body: Spanned<Expression<'db>>,
}
impl<'db> Function<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src
    {
        let name = FunctionId::parser(db).spanned();

        let type_annotation = just(Token::DPoint)
            .ignore_then(Type::parser(db).spanned())
            .or_not();

        let arg = Pattern::parser(db).spanned().then(type_annotation);

        let args = arg
            .separated_by(just(Token::Comma))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(just(Token::LParen), just(Token::RParen));

        let return_type = just(Token::Minus)
            .ignore_then(just(Token::GreaterThan))
            .ignore_then(Type::parser(db).spanned())
            .or_not();

        just(Token::Fn)
            .ignore_then(name)
            .then(args)
            .then(return_type)
            .then_ignore(just(Token::Equals))
            .then(Expression::parser(db).spanned())
            .then_ignore(just(Token::Semicolon))
            .map(|(((name, args), rty), body)| Self::new(db, name, args, rty, body))

    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::{lex_source, LexedSource}, stream::Stream, GraphingDatabase, ProgramSource};

    use super::*;

    #[salsa::tracked]
    fn test_compile_function<'db>(db: &'db dyn salsa::Database, ls: LexedSource<'db>) -> Function<'db> {
        let tokenstream = ls.tokens(db);
        let stream = Stream::from_iter(tokenstream.to_owned().into_iter());
        Function::parser(db).parse(stream).unwrap()
    }

    #[test]
    fn function(){
        let dbs = GraphingDatabase::default();
        let code = "fn fib(0) = 1;";
        let code = ProgramSource::new(&dbs, code.to_owned());
        let lexed = lex_source(&dbs, code);
        let _ = test_compile_function(&dbs, lexed);
    }
    #[test]
    fn typed_function(){
        let dbs = GraphingDatabase::default();
        let code = "fn fib(x: u8) -> u8 = x + 2;";
        let code = ProgramSource::new(&dbs, code.to_owned());
        let lexed = lex_source(&dbs, code);
        let _ = test_compile_function(&dbs, lexed);
    }
}

