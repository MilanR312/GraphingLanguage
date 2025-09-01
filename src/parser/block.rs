use chumsky::{input::ValueInput, prelude::*};
use crate::{lexer::Token, spanned::{Span, Spanned, SpannedParser}};

use super::{expression::Expression, statement::Statement};


#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub struct BlockExpression<'db>{
    statements: Vec<Spanned<Statement<'db>>>,
    /// the return expression at the end of a block, the expression here is returned from the block
    return_expr: Option<Spanned<Expression<'db>>>
}
impl<'db> BlockExpression<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src
    {
        let statements = Statement::parser(db)
            .spanned()
            .repeated()
            .collect::<Vec<_>>();

        let return_expr = Expression::parser(db)
            .spanned()
            .or_not();

        statements.then(return_expr)
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .map(|(statements, return_expr)| Self {
                statements,
                return_expr
            })
    }
}
#[cfg(test)]
mod tests {
    use crate::{lexer::{lex_source, LexedSource}, stream::Stream, GraphingDatabase, ProgramSource};

    use super::*;

    #[salsa::tracked]
    fn test_compile_block<'db>(db: &'db dyn salsa::Database, ls: LexedSource<'db>) -> BlockExpression<'db> {
        let tokenstream = ls.tokens(db);
        let stream = Stream::from_iter(tokenstream.to_owned().into_iter());
        BlockExpression::parser(db).parse(stream).unwrap()
    }

    #[test]
    fn block_expr(){
        let code = r#"
        {
            let a = 5;
            let b = 3;
            a * b
        }
        "#;
        let db = GraphingDatabase::default();
        let source = ProgramSource::new(&db, code.to_owned());
        let lexed = lex_source(&db, source);
        let expr = test_compile_block(&db, lexed);
        assert_eq!(expr.statements.len(), 2);
        assert!(expr.return_expr.is_some());
    }

}