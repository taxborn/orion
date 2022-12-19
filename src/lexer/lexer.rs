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

    fn to_span(&self) -> Span {
        Span::empty()
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
                "(" => self.append_token_to(&mut buf, slice, TokenKind::LPar),
                ")" => self.append_token_to(&mut buf, slice, TokenKind::LPar),
                "[" => self.append_token_to(&mut buf, slice, TokenKind::LBracket),
                "]" => self.append_token_to(&mut buf, slice, TokenKind::RBracket),
                "{" => self.append_token_to(&mut buf, slice, TokenKind::LBrace),
                "}" => self.append_token_to(&mut buf, slice, TokenKind::RBrace),
                "=" => self.append_token_to(&mut buf, slice, TokenKind::Eq),
                ":" => match self.cursor.peek() {
                    Some(":") => self.append_token_to(&mut buf, "::", TokenKind::ColonColon),
                    Some("=") => self.append_token_to(&mut buf, ":=", TokenKind::UntypedAssignment),
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Colon),
                },
                ";" => self.append_token_to(&mut buf, slice, TokenKind::Semi),
                "$" => self.append_token_to(&mut buf, slice, TokenKind::Dollar),
                "," => self.append_token_to(&mut buf, slice, TokenKind::Comma),
                "-" => match self.cursor.peek() {
                    Some("-") => self.append_token_to(&mut buf, "--", TokenKind::Decrement),
                    Some(">") => self.append_token_to(&mut buf, "->", TokenKind::RightArrow),
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Minus),
                },
                "+" => match self.cursor.peek() {
                    Some("+") => self.append_token_to(&mut buf, "++", TokenKind::Increment),
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Plus),
                },
                "." => match self.cursor.peek() {
                    Some(".") => self.append_token_to(&mut buf, "..", TokenKind::DotDot),
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Dot),
                },
                "~" => self.append_token_to(&mut buf, slice, TokenKind::Tilde),
                "*" => self.append_token_to(&mut buf, slice, TokenKind::Star),
                "/" => match self.cursor.peek() {
                    Some("/") => {
                        // TODO: Single line comment
                        self.cursor.next();
                        //buf.push_back(Token::new("..", TokenKind::DotDot, self.cursor.to_span()));
                    }
                    Some("*") => {
                        // TODO: Multi line comment
                        self.cursor.next();
                        //buf.push_back(Token::new("..", TokenKind::DotDot, self.cursor.to_span()));
                    }
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Slash),
                },
                "%" => self.append_token_to(&mut buf, slice, TokenKind::Percent),
                "&" => self.append_token_to(&mut buf, slice, TokenKind::Ampersand),
                "|" => self.append_token_to(&mut buf, slice, TokenKind::Bar),
                "^" => self.append_token_to(&mut buf, slice, TokenKind::Hat),
                ">" => match self.cursor.peek() {
                    Some(">") => self.append_token_to(&mut buf, ">>", TokenKind::GreaterGreater),
                    Some("=") => self.append_token_to(&mut buf, ">=", TokenKind::GreaterEq),
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Greater),
                },
                "<" => match self.cursor.peek() {
                    Some("<") => self.append_token_to(&mut buf, "<<", TokenKind::LesserLesser),
                    Some("=") => self.append_token_to(&mut buf, "<=", TokenKind::LesserEq),
                    Some("-") => self.append_token_to(&mut buf, "<-", TokenKind::LeftArrow),
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Lesser),
                },
                "!" => match self.cursor.peek() {
                    Some("=") => self.append_token_to(&mut buf, "!=", TokenKind::BangEq),
                    _ => self.append_token_to(&mut buf, slice, TokenKind::Bang),
                },
                _ => err = true,
            }

            if err {
                return Err(OrionError::UnknownSlice(slice));
            }
        }

        Ok(buf)
    }

    fn append_token_to<'b>(
        &mut self,
        buffer: &mut VecDeque<Token<'b>>,
        slice: &'b str,
        kind: TokenKind,
    ) {
        let token = Token::new(slice, kind, self.cursor.to_span());

        self.cursor.next_by(token.length());

        buffer.push_back(token);
    }
}
