mod literal;

use super::types;
use super::read_types;

pub use self::literal::*;

use super::types::*;
use super::read_types::*;

#[derive(Debug)]
pub struct Map<P, F> {
    pub(super) inner: P,
    pub(super) func: F
}

impl <'a, P, F, U> Parser<'a> for Map<P, F>
    where P: Parser<'a>,
          F: Fn(P::Item) -> U
{
    type Item = U;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        self.inner.parse(source).map(|(value, source)| ((self.func)(value), source))
    }
}

#[derive(Debug)]
pub struct Optional<P> {
    pub(super) inner: P
}

impl <'a, P> Parser<'a> for Optional<P>
    where P: Parser<'a>
{
    type Item = Option<P::Item>;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        match self.inner.parse(source) {
            Ok((value, source)) => Ok((Some(value), source)),
            Err(_) => Ok((None, Source {
                inner: source.inner,
                position: source.position
            }))
        }
    }
}

#[derive(Debug)]
pub struct Delimited<P, D> {
    pub(super) inner: P,
    pub(super) delim: D
}

impl <'a, P, D> Parser<'a> for Delimited<P, D>
    where P: Parser<'a>,
          D: Parser<'a>
{
    type Item = Vec<P::Item>;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        let mut items = vec![];

        let (item, mut next) = self.inner.parse(source)?;
        items.push(item);

        loop {
            if self.delim.parse(&next).is_err() {
                break;
            }

            match self.inner.parse(&next) {
                Ok((item, _next)) => {
                    items.push(item);
                    next = _next;
                },
                Err(_) => break
            }
        }

        Ok((items, next))
    }
}

#[derive(Debug)]
pub struct WhitespaceDelimited<P> {
    pub(super) inner: P
}

impl <'a, P> Parser<'a> for WhitespaceDelimited<P>
    where P: Parser<'a>
{
    type Item = Vec<P::Item>;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        let mut items = vec![];

        let (item, mut next) = self.inner.parse(source)?;
        items.push(item);

        loop {
            next.skip_whitespace();

            match self.inner.parse(&next) {
                Ok((item, _next)) => {
                    items.push(item);
                    next = _next;
                },
                Err(_) => break
            }
        }

        Ok((items, next))
    }
}

#[derive(Debug)]
pub struct Surrounded<P, L, R> {
    pub(super) inner: P,
    pub(super) left: L,
    pub(super) right: R
}

impl <'a, P, L, R> Parser<'a> for Surrounded<P, L, R>
    where P: Parser<'a>,
          L: Parser<'a>,
          R: Parser<'a>
{
    type Item = (L::Item, P::Item, R::Item);

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        let (left, next) = self.left.parse(source)?;
        let (item, next) = self.inner.parse(&next)?;
        let (right, next) = self.right.parse(&next)?;

        Ok(((left, item, right), next))
    }
}

#[derive(Debug)]
pub struct Or<L, R> {
    pub(super) left: L,
    pub(super) right: R
}

impl <'a, L, R> Parser<'a> for Or<L, R>
    where L: Parser<'a>,
          R: Parser<'a>
{
    type Item = Either<L::Item, R::Item>;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        if let Ok((item, next)) = self.left.parse(source) {
            Ok((Either::Left(item), next))
        } else {
            self.right.parse(source).map(|(item, next)| (Either::Right(item), next))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct WithSpanParser<P> {
    pub(super) inner: P
}

impl <'a, P> Parser<'a> for WithSpanParser<P> 
    where P: Parser<'a>
{
    type Item = WithSpan<P::Item>;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        let ( item, next ) = self.inner.parse(source)?;
        let len = source.inner.len() - next.inner.len();

        let span = Span::new(
            source.position,
            source.position.advance(&source.inner[..len])
        );

        Ok((WithSpan {
            value: item,
            span
        }, next))
    }
}