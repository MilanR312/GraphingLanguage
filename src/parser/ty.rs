use crate::{ids::TypeId, lexer::Token, spanned::{Span, Spanned, SpannedParser}};
use chumsky::{input::ValueInput, prelude::*};


#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub enum Type<'db>{
    /// An inferred type _
    Inferred(Spanned<()>),
    /// A regular type like u8
    Type(Spanned<TypeId<'db>>),
    /// A tuple type like (u8, _)
    Tuple(Vec<Spanned<Type<'db>>>),
    // TODO: array [u8; 5]
}
impl<'db> Type<'db> {
    pub fn parser<'src, I>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 
    'db: 'src,
    I: ValueInput<'src, Span = Span, Token = Token>
    {
        recursive(move |atom| {
            let inferred = just(Token::Wildcard)
                .ignored()
                .spanned()
                .map(|x| Self::Inferred(x));

            let ty = TypeId::parser(db)
                .spanned()
                .map(|x| Self::Type(x));

            let tuple = atom
                .spanned()
                .separated_by(just(Token::Comma))
                .at_least(1)
                .collect::<Vec<_>>()
                .delimited_by(just(Token::LParen), just(Token::RParen))
                .map(|x| Type::Tuple(x));

            choice((
                inferred,
                ty,
                tuple
            ))
        })
    }
}


#[cfg(test)]
mod type_test {
    use crate::{lexer::lex_to_stream, GraphingDatabase};

    use super::*;
    #[test]
    fn wildcard(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("_");
        let out = Type::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Type::Inferred(_)));
    }
    #[test]
    fn variable(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("a");
        let out = Type::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Type::Type(_)));
    }
    #[test]
    fn tuple(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("(a, _)");
        let out = Type::parser(&dbs).parse(src).unwrap();
        let Type::Tuple(x) = out else { panic!() };
        assert_eq!(x.len(), 2);
    }
}