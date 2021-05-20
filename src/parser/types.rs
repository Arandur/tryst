use lazy_static::lazy_static;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct FilePosition {
    pub line: usize,
    pub col: usize,
}

impl FilePosition {
    pub fn span(self, text: &str) -> FileSpan {
        FileSpan {
            start: self,
            end: self.advance(text)
        }
    }

    fn advance(self, text: &str) -> FilePosition {
        let mut new_position = self;
        let mut skip_next_lf = false;

        for c in text.chars() {
            match c {
                '\x0a' => if !skip_next_lf {
                    new_position.advance_line();
                    skip_next_lf = false;
                },
                '\x0d' => {
                    new_position.advance_line();
                    skip_next_lf = true;
                },
                '\x0b' | '\x0c' | '\u{85}' | '\u{2028}' | '\u{2029}' => {
                    new_position.advance_line()
                },
                _ => new_position.advance_bytes(c)
            }
        }

        new_position
    }

    /* 
     * TODO?: separate col into 'byte' and 'col', and make a best-effort
     * attempt to keep track of "character" position, i.e. the column showing
     * the character in a monospace text editor.
     */

    fn advance_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    fn advance_bytes(&mut self, c: char) {
        self.col += c.len_utf8();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct FileSpan {
    pub start: FilePosition,
    pub end: FilePosition
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct ParserInput<'a> {
    pub value: &'a str,
    pub position: FilePosition
}

impl <'a> ParserInput<'a> {
    pub fn skip_whitespace(&mut self) {
        if let Ok(ParserOutput { next, .. }) = parse_whitespace(&self) {
            *self = next;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(super) struct ParserOutput<'a, T> {
    pub value: T,
    pub span: FileSpan,
    pub next: ParserInput<'a>
}

impl <'a, T> ParserOutput<'a, T> {
    pub fn map<F: Fn(T) -> U, U>(self, func: F) -> ParserOutput<'a, U> {
        ParserOutput {
            value: func(self.value),
            span: self.span,
            next: self.next
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct ParseError {
    pub kind: ErrorKind,
    pub position: FilePosition
}

#[derive(Debug, PartialEq)]
pub(super) enum ErrorKind {
    Expected {
        expected: &'static str
    }
}

pub(super) type ParseResult<'a, T> = Result<ParserOutput<'a, T>, ParseError>;

pub(super) trait Parser<'a> {
    type Item;

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item>;
}

impl <'a, T, F> Parser<'a> for F
    where F: Fn(&ParserInput<'a>) -> ParseResult<'a, T>,
          T: 'a
{
    type Item = T;

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item> 
        where Self::Item: 'a 
    {
        self(input)
    }
}

pub(super) use crate::types::Atom;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Form {
    Atom(Atom),
    List(List)
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct List(pub Vec<Form>);

pub(super) struct LiteralParser {
    pub value: &'static str
}

impl LiteralParser {
    pub(super) fn new(value: &'static str) -> LiteralParser {
        LiteralParser { value }
    }
}

impl <'a> Parser<'a> for LiteralParser {
    type Item = &'static str;

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item> {
        let ParserInput { value, position } = input;
        let span = position.span(self.value);

        if let Some(rem) = value.strip_prefix(self.value) {
            Ok(ParserOutput {
                value: self.value,
                span,
                next: ParserInput { value: rem, position: span.end }
            })
        } else {
            Err(ParseError {
                kind: ErrorKind::Expected {
                    expected: self.value
                },
                position: *position
            })
        }
    }
}

use regex::Regex;

pub(super) struct RegexParser<'r> {
    pub regex: &'r Regex,
    pub description: &'static str
}

impl <'a, 'r> Parser<'a> for RegexParser<'r> {
    type Item = &'a str;

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item> 
        where Self::Item: 'a
    {
        let ParserInput { value, position } = input;

        if let Some(m) = self.regex.find(value) {
            let (item, next) = value.split_at(m.end());
            let span = position.span(item);

            Ok(ParserOutput {
                value: item,
                span,
                next: ParserInput {
                    value: next,
                    position: span.end
                }
            })
        } else {
            Err(ParseError {
                kind: ErrorKind::Expected {
                    expected: self.description
                },
                position: *position
            })
        }
    }
}

pub(super) struct ConcatParser<'a, 'c> {
    chain: &'c [&'c dyn Parser<'a, Item=&'a str>]
}

impl <'a, 'c> ConcatParser<'a, 'c> {
    pub fn new(chain: &'c [&'c dyn Parser<'a, Item=&'a str>]) -> ConcatParser<'a, 'c> {
        ConcatParser { chain }
    }
}

impl <'a, 'c> Parser<'a> for ConcatParser<'a, 'c> {
    type Item = &'a str;

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item> {
        let ParserInput { value, position } = input;

        if let Some((first, rest)) = self.chain.split_first() {
            let ParserOutput { 
                value: first_value,
                span: first_span, 
                next
            } = first.parse(input)?;

            let ParserOutput { 
                value: rest_value,
                span: rest_span,
                next: rest_next,
            } = ConcatParser { chain: rest }.parse(&next)?;

            Ok(ParserOutput {
                value: &value[..(first_value.len() + rest_value.len())],
                span: FileSpan {
                    start: first_span.start,
                    end: rest_span.end
                },
                next: rest_next
            })
        } else {
            Ok(ParserOutput {
                value: "",
                span: FileSpan { start: *position, end: *position },
                next: *input
            })
        }
    }
}

pub(super) struct MaybeParser<'a, 'p, T> {
    parser: &'p dyn Parser<'a, Item=T>
}

impl <'a, 'p, T> MaybeParser<'a, 'p, T> {
    pub fn new(parser: &'p dyn Parser<'a, Item=T>) -> MaybeParser<'a, 'p, T> {
        MaybeParser { parser }
    }
}

impl <'a, 'p, T> Parser<'a> for MaybeParser<'a, 'p, T> {
    type Item = Option<T>;

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item> {
        self.parser.parse(input)
            .map(|result| ParserOutput {
                value: Some(result.value),
                span: result.span,
                next: result.next
            })
            .or_else(|_| Ok(ParserOutput {
                value: None,
                span: FileSpan {
                    start: input.position,
                    end: input.position
                },
                next: *input
            }))
    }
}

pub(super) struct ChainParser<'a, 'p, 'q, T, U> {
    left: &'p dyn Parser<'a, Item=T>,
    right: &'q dyn Parser<'a, Item=U>
}

impl <'a, 'p, 'q, T, U> ChainParser<'a, 'p, 'q, T, U> {
    pub fn new(
        left: &'p dyn Parser<'a, Item=T>,
        right: &'q dyn Parser<'a, Item=U>
    ) -> ChainParser<'a, 'p, 'q, T, U> {
        ChainParser { left, right }
    }
}

impl <'a, 'p, 'q, T, U> Parser<'a> for ChainParser<'a, 'p, 'q, T, U> {
    type Item = (T, U);

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item> {
        self.left.parse(input)
            .and_then(|ParserOutput { value: left_value, span: left_span, next }| {
                self.right.parse(&next)
                    .map(|ParserOutput { value: right_value, span: right_span, next }| {
                        ParserOutput {
                            value: (left_value, right_value),
                            span: FileSpan {
                                start: left_span.start,
                                end: right_span.end
                            },
                            next
                        }
                    })
            })
    }
}

pub(super) struct RepeatParser<'a, 'p, T> {
    pub parser: &'p dyn Parser<'a, Item=T>
}

impl <'a, 'p, T> RepeatParser<'a, 'p, T> {
    pub fn new(parser: &'p dyn Parser<'a, Item=T>) -> RepeatParser<'a, 'p, T> {
        RepeatParser { parser }
    }
}

impl <'a, 'p, T> Parser<'a> for RepeatParser<'a, 'p, T> {
    type Item = Vec<T>;

    fn parse(&self, input: &ParserInput<'a>) -> ParseResult<'a, Self::Item> {
        let ParserOutput { value, mut span, mut next } = self.parser.parse(input)?;
        let mut items = vec![value];

        next.skip_whitespace();

        while let Ok(ParserOutput { value, span: s, next: n }) = self.parser.parse(&next) {
            items.push(value);
            span.end = s.end;
            next = n;

            next.skip_whitespace();
        }

        Ok(ParserOutput {
            value: items,
            span,
            next
        })
    }
}

fn parse_whitespace<'a>(input: &ParserInput<'a>) -> ParseResult<'a, ()> {
    lazy_static! {
        static ref WS: Regex = Regex::new(
            r"^(?m:\s|\n)+"
        ).unwrap();
    }

    RegexParser { regex: &WS, description: "whitespace" }
        .parse(input)
        .map(|o| o.map(|_| ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_parser() {
        let input = ParserInput {
            value: "(())",
            position: FilePosition { line: 1, col: 0 }
        };

        // Empty parser
        let result = ConcatParser::new(&[]).parse(&input);

        assert_eq!(
            result,
            Ok(ParserOutput {
                value: "",
                span: FileSpan {
                    start: FilePosition { line: 1, col: 0 },
                    end: FilePosition { line: 1, col: 0 }
                },
                next: ParserInput {
                    value: "(())",
                    position: FilePosition { line: 1, col: 0 }
                }
            })
        );

        // Parser with one element
        let result = ConcatParser::new(&[
            &LiteralParser::new("(")
        ]).parse(&input);

        assert_eq!(
            result,
            Ok(ParserOutput {
                value: "(",
                span: FileSpan {
                    start: FilePosition { line: 1, col: 0 },
                    end: FilePosition { line: 1, col: 1 }
                },
                next: ParserInput {
                    value: "())",
                    position: FilePosition { line: 1, col: 1 }
                }
            })
        );

        // Parser with many elements
        let result = ConcatParser::new(&[
            &LiteralParser::new("("),
            &LiteralParser::new("("),
            &LiteralParser::new(")"),
            &LiteralParser::new(")")
        ]).parse(&input);

        assert_eq!(
            result,
            Ok(ParserOutput {
                value: "(())",
                span: FileSpan {
                    start: FilePosition { line: 1, col: 0 },
                    end: FilePosition { line: 1, col: 4 }
                },
                next: ParserInput {
                    value: "",
                    position: FilePosition { line: 1, col: 4 }
                }
            })
        );
    }

    #[test]
    fn test_maybe() {
        let input = ParserInput {
            value: "a",
            position: FilePosition { line: 1, col: 0 }
        };

        let result = MaybeParser::new(&LiteralParser::new("a")).parse(&input);

        assert_eq!(
            result,
            Ok(ParserOutput {
                value: Some("a"),
                span: FileSpan {
                    start: FilePosition { line: 1, col: 0 },
                    end: FilePosition { line: 1, col: 1 }
                },
                next: ParserInput {
                    value: "",
                    position: FilePosition { line: 1, col: 1 }
                }
            })
        );

        let input = ParserInput {
            value: "",
            position: FilePosition { line: 1, col: 0 }
        };

        let result = MaybeParser::new(&LiteralParser::new("b")).parse(&input);

        assert_eq!(
            result,
            Ok(ParserOutput {
                value: None,
                span: FileSpan {
                    start: FilePosition { line: 1, col: 0 },
                    end: FilePosition { line: 1, col: 0 }
                },
                next: ParserInput {
                    value: "",
                    position: FilePosition { line: 1, col: 0 }
                }
            })
        )
    }

    #[test]
    fn test_skip_whitespace() {
        let mut input = ParserInput {
            value: "   \n   3",
            position: FilePosition { line: 1, col: 0 }
        };

        input.skip_whitespace();

        assert_eq!(
            input,
            ParserInput {
                value: "3",
                position: FilePosition { line: 2, col: 3 }
            }
        )
    }
}