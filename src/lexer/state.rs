//! Lexer for the Orion compiler
use crate::lexer::tokens::Location;
use crate::lexer::tokens::Token;
use crate::lexer::tokens::TokenKind;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug)]
pub struct Lexer<'a> {
    // The initial input to the lexer
    input: &'a str,
    // A peekable iterator of all characters plus their indicies.
    // TODO, it seems like Peekable<> makes things slow to remove Peekable<>
    // and use clones of the iterator: https://gist.github.com/eliben/a6a2a55a33e733e3104827ab03ebc720
    iter: Peekable<CharIndices<'a>>,

    // the last character taken from the iterator
    c: char,
    // the index of the previous character
    ci: usize,

    // true if and only if lexing encountered an error. this hopefully will
    // be replaced by using Results, but unsure how to best handle that
    // witn an iterator
    pub error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lex = Self {
            input,
            iter: input.char_indices().peekable(),
            // null character
            c: '\x00',
            ci: 0,
            error: false,
        };

        // scan the first character
        lex.scan_char();

        lex
    }

    fn scan_char(&mut self) {
        // Check if the iterator has something, if so, scan the next character
        if let Some((index, chr)) = self.iter.next() {
            // Update the index
            self.ci = index;
            // Update the character
            self.c = chr;
        } else {
            // Otherwise, we are at the end. Set the index to the end of the
            // buffer
            self.ci = self.input.len();
            // And set the current character to a null character
            self.c = '\x00';
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        // Skip everything at the beginning that we don't need. Spaces, tabs,
        // etc..
        self.skip_nontokens();
        // Get the current location of the token
        let loc = Location::from_input(&self.input[..self.ci]);

        // If we are at the end, return EOF token
        if self.is_at_end() {
            return Token {
                kind: TokenKind::Eof,
                loc,
            };
        }

        // Otherwise, match against different symbols
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
                        self.skip_n(2);
                        return Token {
                            kind: TokenKind::UntypedAssignment,
                            loc,
                        };
                    } else if *chr == ':' {
                        self.skip_n(2);
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
                        self.skip_n(2);
                        return Token {
                            kind: TokenKind::RightArrow,
                            loc,
                        };
                    } else if *chr == '-' {
                        self.skip_n(2);
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
                        self.skip_n(2);
                        return Token {
                            kind: TokenKind::LeftArrow,
                            loc,
                        };
                    } else if *chr == '<' {
                        self.skip_n(2);
                        return Token {
                            kind: TokenKind::LesserLesser,
                            loc,
                        };
                    } else if *chr == '=' {
                        self.skip_n(2);
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
                        self.skip_n(2);
                        return Token {
                            kind: TokenKind::GreaterGreater,
                            loc,
                        };
                    } else if *chr == '=' {
                        self.skip_n(2);
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
                        self.skip_n(2);
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
                        self.skip_n(2);
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
                        self.skip_n(2);
                        return self.scan_single_line_comment();
                    } else if *chr == '*' {
                        self.skip_n(2);
                        return self.scan_multiline_comment();
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
                        self.skip_n(2);
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
                        self.skip_n(2);
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

        // If we got something, create a token at the position and the given
        // TokenKind
        if kind != TokenKind::Error {
            let token = Token { kind, loc };

            self.scan_char();

            token
        // Otherwise, it may be an identifier. An identifier may start with
        // anything alphabetic or an underscore, and may contain anything
        // alphabetic, underscores, or numbers.
        } else if self.c.is_alphabetic() || self.c == '_' {
            self.scan_identifier()
        // Otherwise, if it starts with a digit, we are lexing a number`
        } else if self.c.is_ascii_digit() {
            self.scan_number()
        // Otherwise, we have encountered an error and must return an error
        // token
        } else {
            self.error_token()
        }
    }

    fn scan_identifier(&mut self) -> Token<'a> {
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..startpos]);

        // Identifiers allow alphabetic characters, underscores, and numerics.
        // We can do this check without also checking that the first character
        // satisfies the criteria since we do that in the lexer `.next_token()`
        // function itself.
        while self.c.is_alphabetic() || self.c == '_' || self.c.is_ascii_digit() {
            self.scan_char();
        }

        // Get the identifier
        let input = &self.input[startpos..self.ci];

        // Checks for hard keywords. Mostly unimplemented for now.
        if input == "let" {
            return Token {
                kind: TokenKind::Let,
                loc,
            };
        }

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
        // Get the starting point of the token for use in the Location
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..startpos]);

        // TOOD: Check and account for various bases (oct, dec, hex)
        while self.c.is_ascii_digit() || self.c == '_' || self.c == '.' {
            self.scan_char();
        }

        Token {
            kind: TokenKind::Number(&self.input[startpos..self.ci]),
            loc,
        }
    }

    fn scan_quote(&mut self) -> Token<'a> {
        // Get the starting point of the token for use in the Location
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..startpos]);

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

    fn scan_single_line_comment(&mut self) -> Token<'a> {
        // Get the starting point of the token for use in the Location
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..self.ci]);

        // Scan until the end of the line
        while !self.is_at_end() && self.c != '\n' {
            self.scan_char();
        }

        Token {
            kind: TokenKind::Comment(&self.input[startpos..self.ci]),
            loc,
        }
    }

    fn scan_multiline_comment(&mut self) -> Token<'a> {
        // Get the starting point of the token for use in the Location
        let startpos = self.ci;
        let loc = Location::from_input(&self.input[..startpos]);
        // A variable to keep track whether the comment was closed
        let mut closed = false;

        while !self.is_at_end() {
            self.scan_char();

            // Check if the current and next tokens is a '*/', a closing
            // multiline comment delimeter.
            if self.c == '*' {
                if let Some((_, chr)) = self.iter.peek() {
                    if *chr == '/' {
                        // consume the trailing */
                        self.skip_n(2);
                        closed = true;
                        break;
                    }
                }
            }
        }

        // If it's closed, return the Token, otherwise, return an error
        if closed {
            Token {
                kind: TokenKind::Comment(&self.input[startpos..(self.ci - 2)]),
                loc,
            }
        } else {
            self.error_token()
        }
    }

    /// Generate an error token
    fn error_token(&mut self) -> Token<'a> {
        // Update to lexer to denote an error
        self.error = true;
        // Get the current location
        let loc = Location::from_input(&self.input[..self.ci]);

        Token {
            kind: TokenKind::Error,
            loc,
        }
    }

    /// Check if the lexer is at the end of it's input
    fn is_at_end(&self) -> bool {
        self.ci >= self.input.len()
    }

    /// Skip the tokens that aren't used in symbols or identifiers
    fn skip_nontokens(&mut self) {
        // We don't care about spaces, tabs, new lines..
        while self.c == ' ' || self.c == '\t' || self.c == '\r' || self.c == '\n' {
            self.scan_char();
        }
    }

    /// When scanning through multi-charactered tokens, sometimes it's useful
    /// to skip two at once.
    fn skip_n(&mut self, many: usize) {
        for i in 0..many {
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
