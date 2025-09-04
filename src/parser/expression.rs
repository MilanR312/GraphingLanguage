use crate::{ids::VariableId, lexer::Token, spanned::{Span, Spanned, SpannedParser}};

use super::{block::BlockExpression, literal::Literal, operations::BinaryOp, statement::Statement};
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
    Block(BlockExpression<'db>),
    /// an if expression if foo { a } else { b }
    If
}
impl<'db> Expression<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(
        db: &'db dyn salsa::Database,
        statement_parser: impl Parser<'src, I, Statement<'db>> + 'src + Clone
    ) -> impl Parser<'src, I, Self> + Clone
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

            let parens = atom.clone()
                .delimited_by(just(Token::LParen), just(Token::RParen));

            let block = BlockExpression::parser(db, atom.clone(), statement_parser)
                .map(|x| Self::Block(x));


            let atom = choice((
                todo,
                literal,
                variable,
                parens,
                block
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
    use crate::{lexer::{lex_source, lex_to_stream}, stream::Stream, GraphingDatabase, ProgramSource};

    use super::*;

    #[salsa::tracked]
    fn compile_expression<'db>(db: &'db dyn salsa::Database, code: ProgramSource) -> Expression<'db> {
        let lexed = lex_source(db, code);
        let tokenstream = lexed.tokens(db);
        let stream = Stream::from_iter(tokenstream.to_owned().into_iter());
        let sp = Statement::parser(db);
        Expression::parser(db, sp).parse(stream).unwrap()
    }

    #[test]
    fn todo(){
        let dbs = GraphingDatabase::default();
        let code = ProgramSource::new(&dbs, "_".to_owned());
        let out = compile_expression(&dbs, code);
        assert!(matches!(out, Expression::Todo(_)));
    }
    #[test]
    fn variable(){
        let dbs = GraphingDatabase::default();
        let code = ProgramSource::new(&dbs, "a".to_owned());
        let out = compile_expression(&dbs, code);
        assert!(matches!(out, Expression::Variable(_)));
    }
    #[test]
    fn literal(){
        let dbs = GraphingDatabase::default();
        let code = ProgramSource::new(&dbs, "20".to_owned());
        let out = compile_expression(&dbs, code);
        assert!(matches!(out, Expression::Literal(_)));
    }
    #[test]
    fn add(){
        let dbs = GraphingDatabase::default();
        let code = ProgramSource::new(&dbs, "a + b".to_owned());
        let out = compile_expression(&dbs, code);
        let Expression::Binary(a, b, c) = out else { panic!() };
        assert_eq!(*b, BinaryOp::Add);
    }
    #[test]
    fn paren(){
        let dbs = GraphingDatabase::default();
        let code = ProgramSource::new(&dbs, "(a + b) * c".to_owned());
        let out = compile_expression(&dbs, code);
        let Expression::Binary(a, b, c) = out else { panic!() };
        assert_eq!(*b, BinaryOp::Multiply);
        let Expression::Binary(a, b, c) = &**a else { panic!() };
        assert_eq!(**b, BinaryOp::Add);
    }
    #[test]
    fn block(){
        let dbs = GraphingDatabase::default();
        let code = r#"{
            let a = 5;
            a * 2
        } * { 5 + 3}"#;
        let code = ProgramSource::new(&dbs, code.to_owned());
        let out = compile_expression(&dbs, code);
        println!("{out:?}");
        let Expression::Binary(l, _, r) = out else {panic!()};
        let Expression::Block(x) = &**l else { panic!() };
        assert_eq!(x.statements(&dbs).len(), 1);
        assert!(x.return_expr(&dbs).is_some());

        let Expression::Block(x) = &**r else { panic!() };
        assert_eq!(x.statements(&dbs).len(), 0);
        assert!(x.return_expr(&dbs).is_some());
    }
}