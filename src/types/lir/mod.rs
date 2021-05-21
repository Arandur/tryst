/**
 * LIR: Low-level Internal Representation
 * 
 * The types in this module are used to construct the interpreter's LIR,
 * and are returned from the read() function.
 */

/**
 * All LIR nodes carry with them their span, indicating the range of bytes
 * in the input string from which the node was parsed.
 */

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

#[derive(Clone, Debug, PartialEq)]
pub struct WithSpan<T> {
    pub value: T,
    pub span: Span
}