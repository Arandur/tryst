mod chars;
mod token;

use chars::{chars, Chars};
use token::{Token, Error as TokenError};

use std::io::Read;

type Result<T> = std::result::Result<T, TokenError>;

pub struct Lexer<R: Read> {
    inner: Chars<R>
}

pub fn lexer<R: Read>(source: R) -> Lexer<R> {
    Lexer { inner: chars(source) }
}

impl <R: Read> Iterator for Lexer<R> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let Lexer { inner, .. } = self;
        Token::next(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parens() {
        let source = "()".as_bytes();
        let mut lexer = lexer(source);

        assert_eq!(lexer.next().unwrap().unwrap(), Token::LeftParen);
        assert_eq!(lexer.next().unwrap().unwrap(), Token::RightParen);
        assert_eq!(lexer.next().is_none(), true);
    }

    #[test]
    fn test_empty() {
        let source = "".as_bytes();
        let mut lexer = lexer(source);

        assert_eq!(lexer.next().is_none(), true);
    }
}