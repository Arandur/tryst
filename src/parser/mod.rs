mod number;
mod list;
mod types;

use lazy_static::lazy_static;
use regex::Regex;

pub fn read(input: &str) -> &str { input }
pub fn eval(input: &str) -> &str { input }
pub fn print(input: &str) -> &str { input }

use types::*;
use crate::types::*;
use number::parse_number;
use list::parse_list;

fn parse_ident<'a>(input: &ParserInput<'a>) -> ParseResult<'a, &'a str> {
    lazy_static! {
        static ref IDENT: Regex = Regex::new(
            r"^\p{XID_Start}\p{XID_Continue}*"
        ).unwrap();
    }

    RegexParser {
        regex: &IDENT,
        description: "an identifier"
    }.parse(input)
}

fn parse_atom<'a>(input: &ParserInput<'a>) -> ParseResult<'a, Atom> {
    if let Ok(output) = parse_ident(input) {
        Ok(output.map(|s| Atom::Symbol(Symbol(s.to_string()))))
    } else {
        Ok(parse_number(input)?.map(Atom::Number))
    }
}

fn parse_form<'a>(input: &ParserInput<'a>) -> ParseResult<'a, Form> {
    if let Ok(output) = parse_atom(input) {
        Ok(output.map(Form::Atom))
    } else {
        Ok(parse_list(input)?.map(Form::List))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ident() {
        let input = ParserInput {
            value: "abc def",
            position: FilePosition { line: 1, col: 0 }
        };
        let result = parse_ident(&input);

        assert_eq!(result, Ok(ParserOutput {
            value: "abc",
            span: FileSpan {
                start: FilePosition { line: 1, col: 0 },
                end: FilePosition { line: 1, col: 3 }
            },
            next: ParserInput {
                value: " def",
                position: FilePosition { line: 1, col: 3 }
            }
        }));
    }

    #[test]
    fn test_parse_ident_colon_end() {
        let input = ParserInput {
            value: "abc: def",
            position: FilePosition { line: 1, col: 0 }
        };
        let result = parse_ident(&input);

        assert_eq!(result, Ok(ParserOutput {
            value: "abc",
            span: FileSpan {
                start: FilePosition { line: 1, col: 0 },
                end: FilePosition { line: 1, col: 3 }
            },
            next: ParserInput {
                value: ": def",
                position: FilePosition { line: 1, col: 3 }
            }
        }));
    }

    #[test]
    fn test_parse_ident_colon_start() {
        let input = ParserInput {
            value: ":abc def",
            position: FilePosition { line: 1, col: 0 }
        };
        let result = parse_ident(&input);

        assert_eq!(result, Err(ParseError {
            kind: ErrorKind::Expected { expected: "an identifier" },
            position: FilePosition { line: 1, col: 0 }
        }));
    }

    #[test]
    fn test_parse_ident_unicode() {
        let input = ParserInput {
            value: "λ def",
            position: FilePosition { line: 1, col: 0 }
        };
        let result = parse_ident(&input);

        assert_eq!(result, Ok(ParserOutput {
            value: "λ",
            span: FileSpan {
                start: FilePosition { line: 1, col: 0 },
                end: FilePosition { line: 1, col: 2 }
            },
            next: ParserInput {
                value: " def",
                position: FilePosition { line: 1, col: 2 }
            }
        }));
    }

    #[test]
    fn test_parse_form() {
        let input = ParserInput {
            value: "(add 2.5 \n  (mul 3 2))",
            position: FilePosition { line: 1, col: 0 }
        };

        let result = parse_form(&input);

        assert_eq!(
            result,
            Ok(ParserOutput {
                value: Form::List(List(vec![
                    Form::Atom(Atom::Symbol(Symbol("add".to_string()))),
                    Form::Atom(Atom::Number(2.5f64)),
                    Form::List(List(vec![
                        Form::Atom(Atom::Symbol(Symbol("mul".to_string()))),
                        Form::Atom(Atom::Number(3.0f64)),
                        Form::Atom(Atom::Number(2.0f64))
                    ]))
                ])),
                span: FileSpan {
                    start: FilePosition { line: 1, col: 0 },
                    end: FilePosition { line: 2, col: 12 }
                },
                next: ParserInput {
                    value: "",
                    position: FilePosition { line: 2, col: 12 }
                }
            })
        );
    }
}