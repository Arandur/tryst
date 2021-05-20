use super::types::*;

use lazy_static::lazy_static;
use regex::Regex;

use std::str::FromStr;

pub(super) fn parse_number<'a>(input: &ParserInput<'a>) -> ParseResult<'a, f64> {
    let output = ChainParser::new(
        &parse_integer,
        &MaybeParser::new(
            &ChainParser::new(
                &LiteralParser::new("."),
                &MaybeParser::new(
                    &ChainParser::new(
                        &parse_integer,
                        &MaybeParser::new(&parse_exponent)
                    )
                )
            )
        )
    ).parse(input)?;

    let prefix = &input.value[..(input.value.len() - output.next.value.len())];

    Ok(ParserOutput {
        value: f64::from_str(prefix).unwrap(),
        span: output.span,
        next: output.next
    })
}

fn parse_integer<'a>(input: &ParserInput<'a>) -> ParseResult<'a, &'a str> {
    lazy_static! {
        static ref INT: Regex = Regex::new(
            r"^[0-9][0-9]*"
        ).unwrap();
    }

    RegexParser {
        regex: &INT,
        description: "an integer literal"
    }.parse(input)
}

fn parse_exponent<'a>(input: &ParserInput<'a>) -> ParseResult<'a, &'a str> {
    lazy_static! {
        static ref EXP: Regex = Regex::new(
            r"^[eE][+-]?[0-9][0-9]*"
        ).unwrap();
    }

    RegexParser {
        regex: &EXP,
        description: "an exponent"
    }.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let input = ParserInput {
            value: "1.0e5)",
            position: FilePosition { line: 1, col: 0 }
        };

        let result = parse_number(&input);

        assert_eq!(
            result,
            Ok(ParserOutput {
                value: 100000.0f64,
                span: FileSpan {
                    start: FilePosition { line: 1, col: 0 },
                    end: FilePosition { line: 1, col: 5 }
                },
                next: ParserInput {
                    value: ")",
                    position: FilePosition { line: 1, col: 5 }
                }
            })
        )
    }
}