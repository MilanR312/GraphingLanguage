use crate::{ids::VariableId, lexer::Token, spanned::{Span, Spanned, SpannedParser}};

use super::literal::Literal;
use chumsky::{input::ValueInput, prelude::*};


/// a pattern type
/// 
/// this is the item shown in let positions and function arguments
/// 
/// let Pattern = foo();
#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub enum Pattern<'db> {
    /// Wildcard pattern _, means the argument is unused
    Wildcard(Spanned<()>),
    /// Variable pattern, this results in regular assignment like let foo = 1;
    Variable(Spanned<VariableId<'db>>),
    /// Literal pattern, this allows pattern matching using functions as follows
    /// 
    /// fn foo(0) = 1;
    /// fn foo(x) = x * foo(x-1)
    /// 
    /// this is not allowed in let positions
    Literal(Spanned<Literal>),
    /// Tuple destructor pattern, allows a tuple to be destructed into parts
    /// 
    /// let (a, b) = _;
    Tuple(Vec<Spanned<Pattern<'db>>>),
}
impl<'db> Pattern<'db> {
    pub fn parser<'src, I>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 
    'db: 'src,
    I: ValueInput<'src, Span = Span, Token = Token>
    {
        recursive(move |atom| {
            let wildcard = just(Token::Wildcard)
                .ignored()
                .spanned()
                .map(|x| Self::Wildcard(x));
            let variable = VariableId::parser(db).spanned().map(|x| Self::Variable(x));
            let literal = Literal::parser().spanned().map(|x| Self::Literal(x));

            let tuple = atom.clone()
                .spanned()
                .separated_by(just(Token::Comma))
                .at_least(1)
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::LParen), just(Token::RParen))
                .map(|x| Self::Tuple(x));


            choice((
                wildcard,
                variable,
                literal,
                tuple
            ))
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::{lexer::lex_to_stream, GraphingDatabase};

    use super::*;
    #[test]
    fn wildcard(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("_");
        let out = Pattern::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Pattern::Wildcard(_)));
    }
    #[test]
    fn variable(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("foo");
        let out = Pattern::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Pattern::Variable(_)));
    }
    #[test]
    fn literal(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("12");
        let out = Pattern::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Pattern::Literal(_)));
    }
    #[test]
    fn tuple(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("(1, _)");
        let out = Pattern::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Pattern::Tuple(_)));
    }
}