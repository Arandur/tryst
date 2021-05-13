mod byte_kind;
mod utf8_char_sink;

/**
 * TODO: Figure out how to pass source position information down the stack
 */

use utf8_char_sink::{Utf8CharSink, Error as EncodingError};

use std::error::Error as StdError;
use std::fmt;
use std::io::{Bytes, Read, Error as IoError};

pub struct Chars<R: Read> {
    inner: Bytes<R>,
    sink: Utf8CharSink
}

pub fn chars<R: Read>(r: R) -> Chars<R> {
    Chars { 
        inner: r.bytes(),
        sink: Utf8CharSink::new()
    }
}

impl <R: Read> Iterator for Chars<R> {
    type Item = Result<char, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let Chars { inner, sink } = self;

        // First byte: if it's None, we can return None. 

        match inner.next()? {
            Ok(b) => {
                if let Some(res) = sink.push(b) {
                    return Some(res.map_err(|e| e.into()));
                }
            },
            Err(e) => return Some(Err(e.into()))
        }

        // Second byte onward: None triggers an Eof error. 

        for _ in 1..4 {
            match inner.next() {
                Some(Ok(b)) => {
                    if let Some(res) = sink.push(b) {
                        return Some(res.map_err(|e| e.into()));
                    }
                },
                Some(Err(e)) => return Some(Err(e.into())),
                None => return Some(Err(Error::Eof))
            }
        }

        unreachable!()
    }
}

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Encoding(EncodingError),
    Eof
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::Encoding(e) => e.fmt(f),
            Error::Eof => write!(f, "Unexpected end of file")
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Encoding(e) => Some(e),
            Error::Eof => None
        }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl From<EncodingError> for Error {
    fn from(e: EncodingError) -> Error {
        Error::Encoding(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let buffer = "app".as_bytes();
        let output: Result<String, Error> = chars(buffer).collect();

        assert_eq!(output.unwrap(), "app".to_string());
    }

    #[test]
    fn test_unicode() {
        let buffer = "Ελλάδα".as_bytes();
        let output: Result<String, Error> = chars(buffer).collect();

        assert_eq!(output.unwrap(), "Ελλάδα".to_string());
    }

    #[test]
    fn test_mid_char_eof() {
        let buffer: &[u8] = &[0xce];
        let output: Result<String, Error> = chars(buffer).collect();

        assert_eq!(output.is_err(), true); // TODO: Find a way to test whether this triggers specifically Eof!!
    }
}