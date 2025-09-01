use std::ops::{Deref, DerefMut};

use chumsky::{extra::ParserExtra, input::Input, Parser};



#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize
}
impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end
        }
    }
    pub fn merge(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end)
        }
    }
}
impl chumsky::span::Span for Span {
    type Context = ();
    type Offset = usize;
    fn new(_context: Self::Context, range: std::ops::Range<Self::Offset>) -> Self {
        Self::new(range.start, range.end)
    }
    fn context(&self) -> Self::Context {
    }
    fn start(&self) -> Self::Offset {
        self.start
    }
    fn end(&self) -> Self::Offset {
        self.end
    }
}



#[derive(PartialEq, Eq, Debug, Hash, salsa::Update, Clone)]
pub struct Spanned<T: PartialEq + salsa::Update> {
    span: Span,
    inner: T
}
impl<T: PartialEq + salsa::Update> Spanned<T> {
    pub fn new(inner: T, span: Span) -> Self {
        Self {
            span,
            inner
        }
    }
    pub fn into_inner(self) -> T {
        self.inner
    }
}
impl<T: PartialEq + salsa::Update> Deref for Spanned<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T: PartialEq + salsa::Update> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


pub trait SpannedParser<'src, I, O, E>: Parser<'src, I, O, E>
where 
    I: Input<'src>,
    E: ParserExtra<'src, I>,
    O: PartialEq + salsa::Update
{
    fn spanned(self) -> impl Parser<'src, I, Spanned<O>, E> + Clone
    where Self: Sized;
}
impl<'src, T, I, O, E> SpannedParser<'src, I, O, E> for T 
where 
    T: Parser<'src, I, O, E> + Clone,
    I: Input<'src, Span = Span>,
    E: ParserExtra<'src, I>,
    O: PartialEq + salsa::Update
{
    fn spanned(self) -> impl Parser<'src, I, Spanned<O>, E> + Clone
        where Self: Sized {
        self.map_with(|x, e| Spanned{
            inner: x,
            span: e.span()
        })
    }
}