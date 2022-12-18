//! Lexer for the Orion compiler
use std::collections::VecDeque;

use crate::error::OrionError;
use crate::lexer::tokens::*;

struct Cursor<'a> {
    input: &'a str,
    current_slice: Option<&'a str>,
    start: usize,
    end: usize,
}

impl<'a> Cursor<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            current_slice: input.get(..1),
            start: 0,
            end: 1,
        }
    }

    fn next(&mut self) {
        self.start += 1;
        self.end += 1;
        self.current_slice = self.input.get(self.start..self.end);
    }

    fn peek(&self) -> Option<&str> {
        self.input.get(self.end..self.end + 1)
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

        while let Some(slice) = self.cursor.current_slice {
            if slice.chars().all(char::is_whitespace) {
                self.cursor.next();
                continue;
            }

            if slice == "+" {
                match self.cursor.peek() {
                    Some("+") => {
                        buf.push_back(Token::new("++", TokenKind::Inc, Span::empty()));
                        self.cursor.next();
                    },
                    _ => {
                        buf.push_back(Token::new(slice, TokenKind::Add, Span::empty()));
                    }
                }

                self.cursor.next();
                continue;
            }

            if slice == "-" {
                match self.cursor.peek() {
                    Some("-") => {
                        buf.push_back(Token::new("--", TokenKind::Dec, Span::empty()));
                        self.cursor.next();
                    },
                    _ => {
                        buf.push_back(Token::new(slice, TokenKind::Add, Span::empty()));
                    }
                }

                self.cursor.next();
                continue;
            }

            return Err(OrionError::UnknownCharacter(slice));
        }

        Ok(buf)
    }
}
