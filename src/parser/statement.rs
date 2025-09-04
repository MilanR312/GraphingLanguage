use std::hash::{Hash, Hasher};

use chumsky::{input::ValueInput, prelude::*};

use crate::{lexer::Token, spanned::Span};

use super::{function::Function, variable::Variable};



// the items in this enum arent spanned due to it being a transparent wrapper, ie: Spanned<Statement<'db>> would be the same as spanning each variant
#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub enum Statement<'db>{
    Function(Function<'db>),
    Variable(Variable<'db>)
}
impl<'db> Statement<'db> {
    pub fn parser<'src, I: ValueInput<'src, Span = Span, Token = Token>>(
        db: &'db dyn salsa::Database,
    ) -> impl Parser<'src, I, Self> + Clone
    where 'db: 'src
    {
        recursive(|p| {
            choice((
                Function::parser(db, p.clone()).map(|x| Self::Function(x)),
                Variable::parser(db, p).map(|x| Self::Variable(x))
            ))
        })
    }

    pub fn hash_id<H: Hasher>(&self, db: &'db dyn salsa::Database, hasher: &mut H) {
        match self {
            Self::Function(x) => (*x.name(db)).hash(hasher),
            Self::Variable(x) => (*x.name(db)).hash(hasher)
        }
    }
}