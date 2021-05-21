pub type Symbol = String;

use crate::read::types::{Source, Error, ErrorKind, Result};

pub fn read<'a>(source: &Source<'a>) -> Result<'a, Symbol> {
    let Source { inner, position } = source;

    // Easier to enumerate the characters a symbol can't start with!
    if inner.starts_with('(') {
        return Err(Error {
            kind: ErrorKind::Expected("a symbol"),
            position: *position
        });
    }

    let end = inner.find(|c: char| c.is_whitespace() || c == ')').unwrap_or_else(|| inner.len());

    if end == 0 {
        return Err(Error {
            kind: ErrorKind::Expected("a symbol"),
            position: *position
        })
    }

    let ( symbol, next ) = inner.split_at(end);
    let next = Source { inner: next, position: position.advance(symbol) };

    Ok((symbol.to_string(), next))
}
