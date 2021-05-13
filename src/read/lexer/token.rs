use super::chars::{Chars, Error as CharsError};

use std::error::Error as StdError;
use std::fmt;
use std::io::Read;

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    LeftParen,
    RightParen
}

impl Token {
    pub fn next<R: Read>(source: &mut Chars<R>) -> Option<Result<Self>> {
        match source.next()? {
            Ok(c) => match c {
                '(' => Some(Ok(Token::LeftParen)),
                ')' => Some(Ok(Token::RightParen)),
                c => Some(Err(Error::UnexpectedChar(c)))
            },
            Err(e) => Some(Err(Error::Chars(e))),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnexpectedChar(char),
    Chars(CharsError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnexpectedChar(c) => write!(f, "Character '{}' does not match any token", c),
            Error::Chars(e) => fmt::Display::fmt(e, f)
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::UnexpectedChar(_) => None,
            Error::Chars(e) => Some(e)
        }
    }
}

impl From<CharsError> for Error {
    fn from(e: CharsError) -> Error {
        Error::Chars(e)
    }
}