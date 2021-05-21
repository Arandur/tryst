use crate::types::lir::{Position};

#[derive(Debug, PartialEq)]
pub struct Source<'a> {
    pub inner: &'a str,
    pub position: Position
}

impl <'a> Source<'a> {
    pub fn new(inner: &'a str) -> Source<'a> {
        Source {
            inner,
            position: Position {
                line: 1,
                col: 0
            }
        }
    }

    pub fn skip_whitespace(&mut self) {
        let Source { inner, position } = self;
        if let Some(pos) = inner.find(|c: char| !c.is_whitespace()) {
            let (ws, rest) = inner.split_at(pos);
            *inner = rest;
            *position = position.advance(ws);
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Position
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Expected(&'static str),
}

pub type Result<'a, T> = core::result::Result<(T, Source<'a>), Error>;
