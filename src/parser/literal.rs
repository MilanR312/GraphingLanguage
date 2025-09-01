use chumsky::{input::ValueInput, prelude::*};
use ordered_float::OrderedFloat;

use crate::{lexer::Token, spanned::{Span, Spanned, SpannedParser}};


/// a parsed literal
#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub enum Literal {
    // any integer signed or not, stored as an u64 but may represent a negative number
    AbstractInt(u64),
    // any float, may represent a f32
    AbstractFloat(OrderedFloat<f64>),
}
impl Literal {
    pub fn parser<'src, I: ValueInput<'src, Token = Token, Span = Span>>() -> impl Parser<'src, I, Self> + Clone
    {
        let atom = select! {
            Token::Int(x) => Self::AbstractInt(x as u64),
            Token::Float(x) => Self::AbstractFloat(x),
        };
        atom
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lex_to_stream;

    use super::*;

    #[test]
    fn unsigned(){
        let src = lex_to_stream("125");
        Literal::parser().parse(src).unwrap();
    }
    #[test]
    fn float(){
        let src = lex_to_stream("1.25");
        Literal::parser().parse(src).unwrap();
    }
    
}