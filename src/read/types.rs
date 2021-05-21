#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub line: usize,
    pub col: usize
}

impl Position {
    pub fn span(self, text: &str) -> Span {
        Span {
            start: self,
            end: self.advance(text)
        }
    }

    pub fn advance(self, text: &str) -> Position {
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
                    new_position.advance_line();
                },
                _ => new_position.advance_bytes(c)
            }
        }

        new_position
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    pub fn advance_bytes(&mut self, c: char) {
        self.col += c.len_utf8();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: Position,
    pub end: Position
}

impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
}

#[derive(Debug, PartialEq)]
pub struct Source<'a> {
    pub inner: &'a str,
    pub position: Position
}

impl <'a> Source<'a> {
    pub fn new(inner: &'a str) -> Source<'a> {
        Source {
            inner,
            position: Position {
                line: 1,
                col: 0
            }
        }
    }

    pub fn skip_whitespace(&mut self) {
        let Source { inner, position } = self;
        if let Some(pos) = inner.find(|c: char| !c.is_whitespace()) {
            let (ws, rest) = inner.split_at(pos);
            *inner = rest;
            *position = position.advance(ws);
        }
    }
}