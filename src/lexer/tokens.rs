#[derive(Debug)]
pub enum TokenKind {
    Add,
    Sub,
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
            "{}:{} -> {}:{}",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.kind, self.span)
    }
}
