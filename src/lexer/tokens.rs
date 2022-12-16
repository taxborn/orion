//! Relevent structures and methods for the Tokens as part of lexical analysis
#[derive(Debug)]
pub enum TokenKind {
    Add,
    Sub,
    Unknown,
}

/// Coordinate type. Used to keep track of (line, column)
type Coordinate = (usize, usize);

#[derive(Debug)]
pub struct Span {
    start: Coordinate,
    end: Coordinate,
}

impl Span {
    pub fn new(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self {
            start: (start_line, start_col),
            end: (end_line, end_col),
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}->{}:{}",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    literal: &'a str,
    kind: TokenKind,
    span: Span,
}

impl<'a> Token<'a> {
    pub fn new(literal: &'a str, kind: TokenKind, span: Span) -> Self {
        Self {
            literal,
            kind,
            span,
        }
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[\"{}\"]@[{}]", self.kind, self.literal, self.span)
    }
}
