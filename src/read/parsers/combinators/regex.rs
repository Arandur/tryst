use super::types::*;
use super::read_types::*;

use regex::Regex;

pub struct RegexParser {
    regex: &'static Regex,
    description: &'static str
}

pub fn regex_parser(regex: &'static Regex, description: &'static str) -> RegexParser {
    RegexParser { regex, description }
}

impl <'a> Parser<'a> for RegexParser {
    type Item = &'a str;

    fn parse(&self, source: &Source<'a>) -> Result<'a, Self::Item> {
        let Source { inner: source, position } = source;
        let RegexParser { regex, description } = self;

        if let Some(m) = regex.find(source) {
            assert_eq!(m.start(), 0, "You forgot to put an anchor on the regex!");

            let (value, rest) = source.split_at(m.end());
            let span = position.span(value);

            Ok((value, Source {
                inner: rest,
                position: span.end
            }))
        } else {
            Err(Error {
                kind: ErrorKind::Expected(description),
                position: *position
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    #[test]
    fn test_regex_parser() {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"\w+(\s+\w+)*"
            ).unwrap();
        }

        let source = Source::new("foo bar baz !!!");
        let result = regex_parser(&RE, "words").parse(&source);

        assert_eq!(
            result,
            Ok(("foo bar baz", Source {
                inner: " !!!",
                position: Position { line: 1, col: 11 }
            }))
        )
    }
}