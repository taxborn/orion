//! Lexer for the Orion compiler
use crate::lexer::tokens::*;
use std::iter::Peekable;
use std::str::CharIndices;

pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,

    c: char,
    ci: usize,

    error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lex = Self {
            input,
            iter: input.char_indices().peekable(),
            c: '\x00',
            ci: 0,
            error: false,
        };

        lex.scan_char();

        lex
    }

    fn scan_char(&mut self) {
        if let Some((index, chr)) = self.iter.next() {
            self.ci = index;
            self.c = chr;
        } else {
            self.ci = self.input.len();
            self.c = '\x00';
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_nontokens();

        if self.is_at_end() {
            return Token {
                kind: TokenKind::Eof,
                loc: Location::empty(),
            };
        }

        let loc = Location::from_input(&self.input[..self.ci]);

        let kind = match self.c {
            '"' => {
                return self.scan_quote();
            }
            '(' => TokenKind::LPar,
            ')' => TokenKind::RPar,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            ':' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '=' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::UntypedAssignment,
                            loc,
                        };
                    } else if *chr == ':' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::ColonColon,
                            loc,
                        };
                    }
                }

                TokenKind::Colon
            }
            ';' => TokenKind::Semi,
            '$' => TokenKind::Dollar,
            ',' => TokenKind::Comma,
            '-' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '>' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::RightArrow,
                            loc,
                        };
                    } else if *chr == '-' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::Decrement,
                            loc,
                        };
                    }
                }

                TokenKind::Minus
            }
            '<' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '-' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::LeftArrow,
                            loc,
                        };
                    } else if *chr == '<' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::LesserLesser,
                            loc,
                        };
                    } else if *chr == '=' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::LesserEq,
                            loc,
                        };
                    }
                }

                TokenKind::Lesser
            }
            '>' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '>' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::GreaterGreater,
                            loc,
                        };
                    } else if *chr == '=' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::GreaterEq,
                            loc,
                        };
                    }
                }

                TokenKind::Greater
            }
            '.' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '.' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::DotDot,
                            loc,
                        };
                    }
                }

                TokenKind::Dot
            }
            '~' => TokenKind::Tilde,
            '+' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '+' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::Increment,
                            loc,
                        };
                    }
                }

                TokenKind::Plus
            }
            '*' => TokenKind::Star,
            '/' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '/' {
                        self.scan_char();
                        self.scan_char();
                        return self.scan_comment();
                    }
                }

                TokenKind::Slash
            }
            '%' => TokenKind::Percent,
            '&' => TokenKind::Ampersand,
            '|' => TokenKind::Bar,
            '^' => TokenKind::Hat,
            '!' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '=' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::BangEq,
                            loc,
                        };
                    }
                }

                TokenKind::Bang
            }
            '=' => {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '=' {
                        self.scan_char();
                        self.scan_char();
                        return Token {
                            kind: TokenKind::EqEq,
                            loc,
                        };
                    }
                }

                TokenKind::Eq
            }
            _ => TokenKind::Error,
        };

        if kind != TokenKind::Error {
            let token = Token { kind, loc };

            self.scan_char();

            token
        } else if self.c.is_alphabetic() || self.c == '_' {
            self.scan_identifier()
        } else if self.c.is_digit(10) {
            self.scan_number()
        } else {
            self.error_token()
        }
    }

    fn scan_identifier(&mut self) -> Token<'a> {
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..self.ci]);

        // TODO: Add support for numbers in identifiers, we already passed
        // the first check.
        while self.c.is_alphabetic() || self.c == '_' {
            self.scan_char();
        }

        let input = &self.input[startpos..self.ci];

        if input == "return" {
            return Token {
                kind: TokenKind::Keyword(input),
                loc,
            };
        }

        Token {
            kind: TokenKind::Identifier(input),
            loc,
        }
    }

    fn scan_number(&mut self) -> Token<'a> {
        let startpos = self.ci;

        while self.c.is_digit(10) {
            self.scan_char();
        }

        let loc = Location::from_input(&self.input[..self.ci]);

        Token {
            kind: TokenKind::Identifier(&self.input[startpos..self.ci]),
            loc,
        }
    }

    fn scan_quote(&mut self) -> Token<'a> {
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..self.ci]);

        // consume leading quote
        self.scan_char();

        while !self.is_at_end() && self.c != '"' {
            self.scan_char();
        }

        if self.c != '"' {
            // Terminating '"' not found is an error
            self.error_token()
        } else {
            // consume trailing quote
            self.scan_char();

            Token {
                kind: TokenKind::Quote(&self.input[startpos..self.ci]),
                loc,
            }
        }
    }

    fn scan_comment(&mut self) -> Token<'a> {
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..self.ci]);

        while !self.is_at_end() && self.c != '\n' {
            self.scan_char();
        }

        Token {
            kind: TokenKind::Comment(&self.input[startpos..self.ci]),
            loc,
        }
    }

    fn error_token(&mut self) -> Token<'a> {
        self.error = true;
        let loc = Location::from_input(&self.input[..self.ci]);

        Token {
            kind: TokenKind::Error,
            loc,
        }
    }

    fn is_at_end(&self) -> bool {
        self.ci >= self.input.len()
    }

    fn skip_nontokens(&mut self) {
        while self.c == ' ' || self.c == '\t' || self.c == '\r' || self.c == '\n' {
            self.scan_char();
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.error {
            return None;
        }

        // Get the next token
        let token = self.next_token();

        // If we are at the end of the file, we don't
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}
