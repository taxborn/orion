use crate::OrionError;

#[derive(Debug)]
pub enum Token {
    Add,
    Sub,
}

#[derive(Debug)]
pub struct Lexer {
    file_contents: String,
    idx: usize,
}

impl Lexer {
    pub fn new(file_contents: String) -> Self {
        Self {
            file_contents,
            idx: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, OrionError> {
        let mut buf = vec![];
        let chars = self.file_contents.chars().filter(|c| c.is_ascii());

        for chr in chars {
            self.idx += 1;

            match chr {
                '+' => buf.push(Token::Add),
                '-' => buf.push(Token::Sub),
                '\n' => continue,
                _ => return Err(OrionError::UnknownCharacter(chr)),
            }
        }

        Ok(buf)
    }

    fn is_eof(&self) -> bool {
        self.idx >= self.file_contents.len()
    }
}
