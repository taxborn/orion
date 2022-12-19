//! Lexer for the Orion compiler
use crate::error::OrionError;
use crate::lexer::tokens::*;
use std::collections::VecDeque;

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

    fn next_by(&mut self, amount: usize) {
        self.start += amount;
        self.end += amount;
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
            let mut err = false;

            if slice.chars().all(char::is_whitespace) {
                self.cursor.next();
                continue;
            }

            match slice {
                "(" => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::LPar, Span::empty()))
                }
                ")" => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::LPar, Span::empty()))
                }
                "[" => self.append_token(
                    &mut buf,
                    Token::new(slice, TokenKind::LBracket, Span::empty()),
                ),
                "]" => self.append_token(
                    &mut buf,
                    Token::new(slice, TokenKind::RBracket, Span::empty()),
                ),
                "{" => self.append_token(
                    &mut buf,
                    Token::new(slice, TokenKind::LBrace, Span::empty()),
                ),
                "}" => self.append_token(
                    &mut buf,
                    Token::new(slice, TokenKind::RBrace, Span::empty()),
                ),
                "=" => self.append_token(&mut buf, Token::new(slice, TokenKind::Eq, Span::empty())),
                ":" => match self.cursor.peek() {
                    Some(":") => self.append_token(
                        &mut buf,
                        Token::new("::", TokenKind::ColonColon, Span::empty()),
                    ),
                    Some("=") => self.append_token(
                        &mut buf,
                        Token::new(":=", TokenKind::UntypedAssignment, Span::empty()),
                    ),
                    _ => self
                        .append_token(&mut buf, Token::new(slice, TokenKind::Colon, Span::empty())),
                },
                ";" => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::Semi, Span::empty()))
                }
                "$" => self.append_token(
                    &mut buf,
                    Token::new(slice, TokenKind::Dollar, Span::empty()),
                ),
                "," => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::Comma, Span::empty()))
                }
                "-" => match self.cursor.peek() {
                    Some("-") => self.append_token(
                        &mut buf,
                        Token::new("--", TokenKind::Decrement, Span::empty()),
                    ),
                    Some(">") => self.append_token(
                        &mut buf,
                        Token::new("->", TokenKind::RightArrow, Span::empty()),
                    ),
                    _ => self
                        .append_token(&mut buf, Token::new(slice, TokenKind::Minus, Span::empty())),
                },
                "+" => match self.cursor.peek() {
                    Some("+") => self.append_token(
                        &mut buf,
                        Token::new("++", TokenKind::Increment, Span::empty()),
                    ),
                    _ => self
                        .append_token(&mut buf, Token::new(slice, TokenKind::Plus, Span::empty())),
                },
                "." => match self.cursor.peek() {
                    Some(".") => self
                        .append_token(&mut buf, Token::new("..", TokenKind::DotDot, Span::empty())),
                    _ => self
                        .append_token(&mut buf, Token::new(slice, TokenKind::Dot, Span::empty())),
                },
                "~" => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::Tilde, Span::empty()))
                }
                "*" => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::Star, Span::empty()))
                }
                "/" => match self.cursor.peek() {
                    Some("/") => {
                        // TODO: Single line comment
                        self.cursor.next();
                        //buf.push_back(Token::new("..", TokenKind::DotDot, Span::empty()));
                    }
                    Some("*") => {
                        // TODO: Multi line comment
                        self.cursor.next();
                        //buf.push_back(Token::new("..", TokenKind::DotDot, Span::empty()));
                    }
                    _ => self
                        .append_token(&mut buf, Token::new(slice, TokenKind::Slash, Span::empty())),
                },
                "%" => self.append_token(
                    &mut buf,
                    Token::new(slice, TokenKind::Percent, Span::empty()),
                ),
                "&" => self.append_token(
                    &mut buf,
                    Token::new(slice, TokenKind::Ampersand, Span::empty()),
                ),
                "|" => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::Bar, Span::empty()))
                }
                "^" => {
                    self.append_token(&mut buf, Token::new(slice, TokenKind::Hat, Span::empty()))
                }
                ">" => match self.cursor.peek() {
                    Some(">") => self.append_token(
                        &mut buf,
                        Token::new(">>", TokenKind::GreaterGreater, Span::empty()),
                    ),
                    Some("=") => self.append_token(
                        &mut buf,
                        Token::new(">=", TokenKind::GreaterEq, Span::empty()),
                    ),
                    _ => self.append_token(
                        &mut buf,
                        Token::new(slice, TokenKind::Greater, Span::empty()),
                    ),
                },
                "<" => match self.cursor.peek() {
                    Some("<") => self.append_token(
                        &mut buf,
                        Token::new("<<", TokenKind::LesserLesser, Span::empty()),
                    ),
                    Some("=") => self.append_token(
                        &mut buf,
                        Token::new("<=", TokenKind::LesserEq, Span::empty()),
                    ),
                    Some("-") => self.append_token(
                        &mut buf,
                        Token::new("<-", TokenKind::LeftArrow, Span::empty()),
                    ),
                    _ => self.append_token(
                        &mut buf,
                        Token::new(slice, TokenKind::Lesser, Span::empty()),
                    ),
                },
                "!" => match self.cursor.peek() {
                    Some("=") => self
                        .append_token(&mut buf, Token::new("!=", TokenKind::BangEq, Span::empty())),
                    _ => self
                        .append_token(&mut buf, Token::new(slice, TokenKind::Bang, Span::empty())),
                },
                _ => err = true,
            }

            if err {
                return Err(OrionError::UnknownSlice(slice));
            }
        }

        Ok(buf)
    }

    fn append_token<'b>(&mut self, buffer: &mut VecDeque<Token<'b>>, token: Token<'b>) {
        self.cursor.next_by(token.length());

        buffer.push_back(token);
    }
}
