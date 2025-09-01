use crate::{ids::VariableId, lexer::Token, spanned::{Span, Spanned, SpannedParser}};

use super::{literal::Literal, operations::BinaryOp};
use chumsky::{input::{MapExtra, ValueInput}, pratt::{infix, left}, prelude::*};


#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub enum Expression<'db> {
    /// a todo expression (_), this expression evaluates to nothing and will never execute, when encountered the interpreter will error instead
    Todo(Spanned<()>),
    /// a literal expression
    Literal(Spanned<Literal>),
    /// a variable expression
    Variable(Spanned<VariableId<'db>>),
    /// a binary expression of the form (a operand b)
    Binary(Box<Spanned<Expression<'db>>>, Spanned<BinaryOp>, Box<Spanned<Expression<'db>>>),
    /// a function call expression foo(a, b)
    FunctionCall,
    /// a block expression let a = { let x = 2; x * 3 }
    Block,
    /// an if expression if foo { a } else { b }
    If
}
impl<'db> Expression<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(db: &'db dyn salsa::Database) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src{
        recursive(move |atom| {
            let todo = just(Token::Wildcard)
                .ignored()
                .spanned()
                .map(|x| Self::Todo(x));

            let literal = Literal::parser::<'src, I>()
                .spanned()
                .map(|x| Self::Literal(x));

            let variable = VariableId::parser(db)
                .spanned()
                .map(|x| Self::Variable(x));

            let parens = atom
                .delimited_by(just(Token::LParen), just(Token::RParen));

            let atom = choice((
                todo,
                literal,
                variable,
                parens
            )).spanned();


            atom.pratt((
                infix(left(2), BinaryOp::multiply().spanned(), |l, op, r, e| {
                    Spanned::new(Self::Binary(Box::new(l), op, Box::new(r)), e.span())
                }),
                infix(left(2), BinaryOp::divide().spanned(), |l, op, r, e| {
                    Spanned::new(Self::Binary(Box::new(l), op, Box::new(r)), e.span())
                }),

                infix(left(1), BinaryOp::add().spanned(), |l, op, r, e| {
                    Spanned::new(Self::Binary(Box::new(l), op, Box::new(r)), e.span())
                }),
                infix(left(1), BinaryOp::subtract().spanned(), |l, op, r, e| {
                    Spanned::new(Self::Binary(Box::new(l), op, Box::new(r)), e.span())
                }),
            )).map(|x| 
                // the infix closure should be able to return an Expr directly according to the docs but when tested the output must be the same as the input (l, r)
                // due to this reason the output is also a spanned expression
                // for consistency we remove the span so all parsers return an unspanned struct
                x.into_inner()
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lex_to_stream, GraphingDatabase};

    use super::*;

    #[test]
    fn todo(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("_");
        let out = Expression::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Expression::Todo(_)));
    }
    #[test]
    fn variable(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("a");
        let out = Expression::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Expression::Variable(_)));
    }
    #[test]
    fn literal(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("a");
        let out = Expression::parser(&dbs).parse(src).unwrap();
        assert!(matches!(out, Expression::Variable(_)));
    }
    #[test]
    fn add(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("a + b");
        let out = Expression::parser(&dbs).parse(src).unwrap();
        let Expression::Binary(a, b, c) = out else { panic!() };
        assert_eq!(*b, BinaryOp::Add);
    }
    #[test]
    fn paren(){
        let dbs = GraphingDatabase::default();
        let src = lex_to_stream("(a + b) * c");
        let out = Expression::parser(&dbs).parse(src).unwrap();
        let Expression::Binary(a, b, c) = out else { panic!() };
        assert_eq!(*b, BinaryOp::Multiply);
        let Expression::Binary(a, b, c) = &**a else { panic!() };
        assert_eq!(**b, BinaryOp::Add);
    }
}