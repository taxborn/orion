//! Relevent structures and methods for the Tokens as part of lexical analysis
use std::fmt::{Display, Formatter, Result, Debug};

#[derive(Debug)]
pub enum TokenKind {
    LPar,              // (
    RPar,              // )
    LBracket,          // [
    RBracket,          // ]
    LBrace,            // {
    RBrace,            // }
    Eq,                // =
    Colon,             // :
    UntypedAssignment, // :=
    Semi,              // ;
    Dollar,            // $
    Comma,             // ,
    RightArrow,        // ->
    LeftArrow,         // <-
    DotDot,            // ..
    Dot,               // .
    Tilde,             // ~
    ColonColon,        // ::

    // Strong keywords
    Return, // return

    // Literals
    Char(char),
    Str(String),
    Identifier,

    // Operators
    Plus,           // +
    Increment,      // ++
    Minus,          // -
    Decrement,      // --
    Star,           // *
    Slash,          // /
    Percent,        // %
    Ampersand,      // &
    Bar,            // |
    Hat,            // ^
    GreaterGreater, // >>
    LesserLesser,   // <<
    Lesser,         // <
    LesserEq,       // <=
    EqEq,           // =
    GreaterEq,      // >=
    Greater,        // >
    BangEq,         // !=
    Bang,           // !

    Eof, // \0 (?)
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

    pub fn empty() -> Self {
        Self {
            start: (0, 0),
            end: (0, 0),
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}:{}->{}:{}",
            self.start.0, self.start.1, self.end.0, self.end.1
        )
    }
}

pub struct Token<'tok> {
    literal: &'tok str,
    kind: TokenKind,
    span: Span,
}

impl<'tok> Token<'tok> {
    pub fn new(literal: &'tok str, kind: TokenKind, span: Span) -> Self {
        Self {
            literal,
            kind,
            span,
        }
    }

    pub fn length(&self) -> usize {
        self.literal.len()
    }

    pub fn identifier(literal: &'tok str, span: Span) -> Self {
        Self {
            literal,
            kind: TokenKind::Identifier,
            span,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "token: {}", self.literal)
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}[\"{}\"]@[{}]", self.kind, self, self.span)
    }
}
