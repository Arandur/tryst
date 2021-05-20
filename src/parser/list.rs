use super::types::*;
use super::parse_form;

pub(super) fn parse_list<'a>(input: &ParserInput<'a>) -> ParseResult<'a, List> {
    let ParserOutput { span, mut next, .. } = parse_open_paren(input)?;
    let start = span.start;

    next.skip_whitespace();

    let ParserOutput { value: forms, mut next, .. } = parse_list_internal(&next)?;

    next.skip_whitespace();

    let ParserOutput { span, next, .. } = parse_close_paren(&next)?;

    let span = FileSpan { start, end: span.end };

    Ok(ParserOutput { value: List(forms), span, next })
}

fn parse_open_paren<'a>(input: &ParserInput<'a>) -> ParseResult<'a, &'static str> {
    LiteralParser { value: "(" }.parse(input)
}

fn parse_close_paren<'a>(input: &ParserInput<'a>) -> ParseResult<'a, &'static str> {
    LiteralParser { value: ")" }.parse(input)
}

fn parse_list_internal<'a>(input: &ParserInput<'a>) -> ParseResult<'a, Vec<Form>> {
    RepeatParser { parser: &parse_form }.parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_open_paren() {
        let input = ParserInput {
            value: "(abc)",
            position: FilePosition { line: 1, col: 0 }
        };
        let result = parse_open_paren(&input);

        assert_eq!(result, Ok(ParserOutput {
            value: "(",
            span: FileSpan {
                start: FilePosition { line: 1, col: 0 },
                end: FilePosition { line: 1, col: 1 }
            },
            next: ParserInput { 
                value: "abc)",
                position: FilePosition { line: 1, col: 1 }
            }
        }));
    }

    #[test]
    fn test_parse_close_paren() {
        let input = ParserInput {
            value: ")",
            position: FilePosition { line: 1, col: 0 }
        };
        let result = parse_close_paren(&input);

        assert_eq!(result, Ok(ParserOutput {
            value: ")",
            span: FileSpan {
                start: FilePosition { line: 1, col: 0 },
                end: FilePosition { line: 1, col: 1 }
            },
            next: ParserInput { 
                value: "",
                position: FilePosition { line: 1, col: 1 }
            }
        }));
    }
}