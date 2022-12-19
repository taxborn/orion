//! Relevent structures and methods for the Tokens as part of lexical analysis
use std::fmt::{Debug, Display, Formatter, Result};

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
    Identifier(String),

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
    GreaterEq,      // >=
    Greater,        // >
    LesserLesser,   // <<
    LesserEq,       // <=
    Lesser,         // <
    EqEq,           // ==
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

    pub fn identifier(literal: &'tok str, ident: String, span: Span) -> Self {
        Self {
            literal,
            kind: TokenKind::Identifier(ident),
            span,
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self.kind {
            TokenKind::LPar => write!(f, "("),
            TokenKind::RPar => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::UntypedAssignment => write!(f, ":="),
            TokenKind::Semi => write!(f, ";"),
            TokenKind::Dollar => write!(f, "$"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::RightArrow => write!(f, "->"),
            TokenKind::LeftArrow => write!(f, "<-"),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::ColonColon => write!(f, "::"),

            TokenKind::Return => write!(f, "return"),

            TokenKind::Char(chr) => write!(f, "Char({chr})"),
            TokenKind::Str(str) => write!(f, "String(\"{str}\")"),
            TokenKind::Identifier(str) => write!(f, "Identifier(\"{str}\")"),

            TokenKind::Plus => write!(f, "+"),
            TokenKind::Increment => write!(f, "++"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Decrement => write!(f, "--"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Bar => write!(f, "|"),
            TokenKind::Hat => write!(f, "^"),
            TokenKind::GreaterGreater => write!(f, ">>"),
            TokenKind::GreaterEq => write!(f, ">="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::LesserLesser => write!(f, "<<"),
            TokenKind::LesserEq => write!(f, "<="),
            TokenKind::Lesser => write!(f, "<"),
            TokenKind::BangEq => write!(f, "!="),
            TokenKind::Bang => write!(f, "!"),
            _ => write!(f, "not implemented"),
        }
    }
}

impl Debug for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}[\"{}\"]@[{}]", self.kind, self, self.span)
    }
}
