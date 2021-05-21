pub type Number = f64;

use crate::read::types::{Source, Error, ErrorKind, Result};

pub fn read<'a>(source: &Source<'a>) -> Result<'a, Number> {
    let Source { inner, position } = source;

    let end = inner.find(|c: char| c.is_whitespace() || c == ')').unwrap_or_else(|| inner.len());
    let ( number, next ) = inner.split_at(end);
    let next = Source { inner: next, position: position.advance(number) };

    number.parse()
        .map(|n| (n, next))
        .map_err(|_| Error {
            kind: ErrorKind::Expected("a number"),
            position: *position
        })
}
