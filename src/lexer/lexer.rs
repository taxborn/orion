//! Lexer for the Orion compiler
use std::collections::VecDeque;

use crate::error::OrionError;
use crate::lexer::tokens::*;
use crate::lexer::*;

struct Cursor<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            position: 0,
            current_char: None,
        }
    }

    fn advance(&mut self) {
        // only increment position if we know we've started
        if self.current_char.is_some() {
            self.position += 1;
        } else {
            println!("ignored incrementing position at init");
        }

        let current = self.input[self.position..].chars().next();

        println!("new current: {current:?}");

        self.current_char = current;
    }

    fn current_char(&self) -> Option<char> {
        self.current_char
    }
}

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let cursor = Cursor::new(input);

        Self { cursor }
    }

    pub fn lex(&mut self) -> Result<VecDeque<Token<'a>>, OrionError> {
        let mut buf = VecDeque::new();

        self.cursor.advance();

        while let Some(chr) = self.cursor.current_char() {
            println!("char found: {chr}");

            match chr {
                '+' => buf.push_back(Token::new("+", TokenKind::Add, Span::empty())),
                '-' => buf.push_back(Token::new("-", TokenKind::Sub, Span::empty())),
                '\n' => continue,
                _ => return Err(OrionError::UnknownCharacter(chr))
            }

            self.cursor.advance();
        }

        Ok(buf)
    }
}
