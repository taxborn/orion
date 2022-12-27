use crate::lexer::state::Lexer;
use crate::lexer::tokens::*;
use crate::parser::ast::*;

pub type Program = Vec<Statement>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Program {
        let mut stmts = vec![];

        while let Some(token) = self.lexer.by_ref().next() {
            match token.kind {
                TokenKind::Let => {
                    let identifier = if let Some(ident) = self.lexer.by_ref().next() {
                        ident
                    } else {
                        panic!("no identifier succeding a let identifier.");
                    };

                    if !matches!(
                        self.lexer.by_ref().next(),
                        Some(Token {
                            kind: TokenKind::Eq,
                            ..
                        })
                    ) {
                        panic!("Assignment expected after let expression");
                    }

                    let expression = self.parse_expression();

                    if !matches!(
                        self.lexer.by_ref().next(),
                        Some(Token {
                            kind: TokenKind::Semi,
                            ..
                        })
                    ) {
                        panic!("Unclosed let assignment");
                    }

                    let stmt = Statement::Let {
                        name: format!("{}", identifier),
                        initial: expression,
                    };

                    stmts.push(stmt);
                }
                _ => unimplemented!(),
            }
        }

        stmts
    }

    fn parse_expression(&mut self) -> Expression {
        match self.lexer.next() {
            Some(Token {
                kind: TokenKind::Number(num),
                ..
            }) => Expression::Number(num.replace("_", "").parse::<f64>().unwrap()),
            Some(Token {
                kind: TokenKind::Identifier(ident),
                ..
            }) => {
                // TODO: Add checking if it is an existing variable, function, etc...
                Expression::Identifier(ident.to_string())
            }
            _ => Expression::Nil,
        }
    }
}
