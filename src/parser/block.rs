use std::hash::DefaultHasher;

use chumsky::{input::ValueInput, prelude::*};
use crate::{ids::BlockId, lexer::Token, spanned::{Span, Spanned, SpannedParser}};

use super::{expression::Expression, statement::Statement};



#[salsa::tracked(debug)]
pub struct BlockExpression<'db>{
    #[returns(ref)]
    pub statements: Vec<Spanned<Statement<'db>>>,
    /// the return expression at the end of a block, the expression here is returned from the block
    #[returns(ref)]
    pub return_expr: Option<Spanned<Expression<'db>>>
}
impl<'db> BlockExpression<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(
        db: &'db dyn salsa::Database,
        expr_parser: impl Parser<'src, I, Expression<'db>> + 'src + Clone,
        statement_parser: impl Parser<'src, I, Statement<'db>> + 'src + Clone,
    ) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src
    {
        let statements = statement_parser
            .spanned()
            .repeated()
            .collect::<Vec<_>>();

        let return_expr = expr_parser
            .spanned()
            .or_not();

        statements.then(return_expr)
            .delimited_by(just(Token::LBrace), just(Token::RBrace))
            .map(|(statements, return_expr)| {
                Self::new(db,statements, return_expr)
            })
    }
}
