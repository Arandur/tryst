use super::types::*;
use super::read_types::Source;

pub struct LiteralParser {
    value: &'static str
}

pub fn literal_parser(value: &'static str) -> LiteralParser {
    LiteralParser { value }
}

impl <'a> Parser<'a> for LiteralParser {
    type Item = &'static str;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        let Source { inner: source, position } = source;
        let span = position.span(self.value);

        if let Some(rest) = source.strip_prefix(self.value) {
            Ok((self.value, Source {
                inner: rest,
                position: span.end
            }))
        } else {
            Err(Error {
                kind: ErrorKind::Expected(self.value),
                position: *position
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::lir::Position;

    #[test]
    fn test_literal_parser() {
        let source = Source::new("foo bar");
        let result = literal_parser("foo").parse(&source);

        assert_eq!(
            result,
            Ok(("foo", Source {
                inner: " bar",
                position: Position { line: 1, col: 3 }
            }))
        )
    }
}