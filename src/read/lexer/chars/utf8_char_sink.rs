use super::byte_kind::ByteKind;

use std::error::Error as StdError;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Utf8CharSink {
    value: u32,
    rem: usize,
}

impl Utf8CharSink {
    pub fn new() -> Utf8CharSink {
        Utf8CharSink { value: 0, rem: 0 }
    }

    pub fn push(&mut self, b: u8) -> Option<Result<char, Error>> {
        let Utf8CharSink { value, rem } = self;

        let expected_start = || -> Option<Result<char, Error>> {
            Some(Err(Error::UnexpectedByte {
                expected: Expected::StartByte,
                actual: b,
            }))
        };

        let expected_continuation = || -> Option<Result<char, Error>> {
            Some(Err(Error::UnexpectedByte {
                expected: Expected::ContByte,
                actual: b,
            }))
        };

        let complete = |v: u32| -> Option<Result<char, Error>> {
            Some(Ok(unsafe { char::from_u32_unchecked(v) }))
        };

        match ByteKind::of(b) {
            ByteKind::Ascii(v) => {
                if *rem == 0 {
                    complete(v)
                } else {
                    expected_continuation()
                }
            }
            ByteKind::Start2(v) => {
                if *rem == 0 {
                    *rem = 1;
                    *value = v << 6;
                    None
                } else {
                    expected_continuation()
                }
            }
            ByteKind::Start3(v) => {
                if *rem == 0 {
                    *rem = 2;
                    *value = v << 12;
                    None
                } else {
                    expected_continuation()
                }
            }
            ByteKind::Start4(v) => {
                if *rem == 0 {
                    *rem = 3;
                    *value = v << 18;
                    None
                } else {
                    expected_continuation()
                }
            }
            ByteKind::Cont(v) => match *rem {
                0 => expected_start(),
                1 => {
                    *rem = 0;
                    let v = *value | v;
                    *value = 0;
                    complete(v)
                }
                2 => {
                    *rem = 1;
                    *value |= v << 6;
                    None
                }
                3 => {
                    *rem = 2;
                    *value |= v << 12;
                    None
                }
                _ => unreachable!(),
            },
            ByteKind::Invalid => match *rem {
                0 => expected_start(),
                _ => expected_continuation()
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    UnexpectedByte { expected: Expected, actual: u8 }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Expected {
    StartByte,
    ContByte
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expected::StartByte => write!(f, "a utf-8 start byte"),
            Expected::ContByte  => write!(f, "a utf-8 continuation byte")
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnexpectedByte { expected, actual } => write!(
                f,
                "Unexpected byte: expected {}, found {}",
                expected, actual
            ),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::UnexpectedByte { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let mut sink = Utf8CharSink::new();

        assert_eq!(sink.push(b'A'), Some(Ok('A')));
    }

    #[test]
    fn test_two_bytes() {
        let mut sink = Utf8CharSink::new();

        assert_eq!(sink.push(0xce), None);
        assert_eq!(sink.push(0xbb), Some(Ok('λ')))
    }

    #[test]
    fn test_three_bytes() {
        let mut sink = Utf8CharSink::new();

        assert_eq!(sink.push(0xe3), None);
        assert_eq!(sink.push(0x83), None);
        assert_eq!(sink.push(0x84), Some(Ok('ツ')));
    }

    #[test]
    fn test_four_bytes() {
        let mut sink = Utf8CharSink::new();

        assert_eq!(sink.push(0xf0), None);
        assert_eq!(sink.push(0x9f), None);
        assert_eq!(sink.push(0xa5), None);
        assert_eq!(sink.push(0xba), Some(Ok('🥺')));
    }

    #[test]
    fn test_invalid_start() {
        let mut sink = Utf8CharSink::new();

        assert_eq!(sink.push(0x90), Some(Err(Error::UnexpectedByte {
            expected: Expected::StartByte,
            actual: 0x90
        })));
    }

    #[test]
    fn test_invalid_cont() {
        let mut sink = Utf8CharSink::new();

        assert_eq!(sink.push(0xf0), None);
        assert_eq!(sink.push(0xf0), Some(Err(Error::UnexpectedByte {
            expected: Expected::ContByte,
            actual: 0xf0
        })));
    }
}
