use crate::read::types::{Source, Result};
use super::combinators;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Either<L, R> {
    Left(L),
    Right(R)
}

pub trait Parser<'a>: Sized {
    type Item;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item>;

    fn map<F: Fn(Self::Item) -> U, U>(self, func: F) -> combinators::Map<Self, F> {
        combinators::Map { inner: self, func }
    }

    fn optional(self) -> combinators::Optional<Self> {
        combinators::Optional { inner: self }
    }

    fn delimited<D>(self, delim: D) -> combinators::Delimited<Self, D> {
        combinators::Delimited { inner: self, delim }
    }

    fn whitespace_delimited(self) -> combinators::WhitespaceDelimited<Self> {
        combinators::WhitespaceDelimited { inner: self }
    }

    fn surrounded<L, R>(self, left: L, right: R) -> combinators::Surrounded<Self, L, R> {
        combinators::Surrounded { inner: self, left, right }
    }

    fn or<R>(self, right: R) -> combinators::Or<Self, R> {
        combinators::Or { left: self, right }
    }

    fn with_span(self) -> combinators::WithSpanParser<Self> {
        combinators::WithSpanParser { inner: self }
    }
}

impl <'a, T: 'a, F> Parser<'a> for F 
    where F: Fn(&Source<'a>) -> Result<'a, T>
{
    type Item = T;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        self(source)
    }
}