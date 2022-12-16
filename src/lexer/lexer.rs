//! Lexer for the Orion compiler
use crate::error::OrionError;
use crate::lexer::tokens::*;

#[derive(Debug)]
struct Cursor {
    input: String,
    position: usize,
    last_position: usize,
}

impl Cursor {
    fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            last_position: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ret = self.input.chars().nth(self.position);

        self.position += 1;

        ret
    }

    fn mark(&mut self) {
        self.last_position = self.position
    }

    fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor,
    current_token: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(file_contents: String) -> Self {
        let cursor = Cursor::new(file_contents);

        Self {
            cursor,
            current_token: None,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, OrionError> {
        let mut buf = vec![];

        while !self.cursor.is_eof() {
            match self.cursor.advance() {
                Some(chr) => match chr {
                    '+' => buf.push(Token::new("x", TokenKind::Add, Span::new(0, 0, 0, 1))),
                    '-' => buf.push(Token::new("x", TokenKind::Sub, Span::new(0, 1, 0, 2))),
                    '\n' => continue,
                    _ => return Err(OrionError::UnknownCharacter(chr)),
                },
                None => break,
            }
        }

        Ok(buf)
    }
}
