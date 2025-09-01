use chumsky::input::{Input, ValueInput};

use crate::Span;

pub struct Stream<I: Iterator> {
    parts: Vec<I::Item>,
    iter: I,
}
impl<I: Iterator> Stream<I> {
    pub fn from_iter<J: IntoIterator<IntoIter = I>>(iter: J) -> Self {
        Self {
            parts: vec![],
            iter: iter.into_iter()
        }
    }
}


impl<'a,T, I> Input<'a> for Stream<I>
where I: Iterator<Item = (T, Span)> + 'a,
    T: 'a + Clone
{
    type Span = Span;
    type Token = T;
    type MaybeToken = T;

    type Cursor = usize;

    type Cache = Self;

    fn begin(self) -> (Self::Cursor, Self::Cache) {
        (0, self)
    }
    fn cursor_location(cursor: &Self::Cursor) -> usize {
        *cursor
    }
    unsafe fn next_maybe(
            cache: &mut Self::Cache,
            cursor: &mut Self::Cursor,
        ) -> Option<Self::MaybeToken> {
        unsafe { Self::next(cache, cursor) }
    }
    unsafe fn span(cache: &mut Self::Cache, range: std::ops::Range<&Self::Cursor>) -> Self::Span {
        let span_a = cache.parts.get(*range.start).map(|x| x.1.start).unwrap_or(0);
        let span_b = cache.parts.get(*range.end - 1).map(|x| x.1.end).unwrap_or(usize::MAX);
        Span::new(span_a, span_b)
    }
}

impl<'a, T, I> ValueInput<'a> for Stream<I>
where I: Iterator<Item = (T, Span)> + 'a,
    T: 'a + Clone
{
    unsafe fn next(cache: &mut Self::Cache, cursor: &mut Self::Cursor) -> Option<Self::Token> {
        if cache.parts.len() <= *cursor {
            cache.parts.extend((&mut cache.iter).take(512));
        }
        cache.parts.get(*cursor).map(|tok| {
            *cursor += 1;
            tok.0.clone()
        })
    }   
}