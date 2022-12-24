//! Relevent structures and methods for the Tokens as part of lexical analysis
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind<'tok> {
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

    // Literals
    Comment(&'tok str),
    Identifier(&'tok str),
    Keyword(&'tok str),
    Number(&'tok str),
    Quote(&'tok str),

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

    Eof,
    Error,
}

pub struct Location(usize, usize);

impl Location {
    fn new(line: usize, col: usize) -> Self {
        Self(line, col)
    }

    pub fn empty() -> Self {
        Self(0, 0)
    }

    pub fn from_input(input: &str) -> Self {
        // TODO: Current bug in computing lines. this just doesn't work.
        let lines: usize = input
            .chars()
            .take_while(|ch| *ch == '\n')
            .count();

        println!("lines found: {lines}\ninput: {input}");

        let cols = match input.rfind('\n') {
            Some(index) => input.len() - index - 1,
            None => input.len()
        };

        Self(lines + 1, cols + 1)
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "L{}:C{}", self.0, self.1)
    }
}

pub struct Token<'tok> {
    pub kind: TokenKind<'tok>,
    pub loc: Location,
}

impl<'tok> Token<'tok> {
    pub fn new(kind: TokenKind<'tok>, loc: Location) -> Self {
        Self { kind, loc }
    }

    pub fn length(&self) -> usize {
        let lexemme = format!("{}", self);

        lexemme.len()
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
            TokenKind::EqEq => write!(f, "=="),
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

            TokenKind::Comment(str) => write!(f, "\"{str}\""),
            TokenKind::Identifier(str) => write!(f, "{str}"),
            TokenKind::Keyword(str) => write!(f, "[{str}]"),
            TokenKind::Number(str) => write!(f, "#{str}"),
            TokenKind::Quote(str) => write!(f, "{str}"),

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
        write!(
            f,
            "{:?}[\"{}\"]@[{}] (sized {})",
            self.kind,
            self,
            self.loc,
            self.length()
        )
    }
}
