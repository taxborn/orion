#[derive(Debug)]
pub enum Token {
    Add,
    Sub,
    Unknown
}

pub enum LexerError {
    UnknownCharacter(char) 
}

#[derive(Debug)]
pub struct Lexer {
    file_contents: String,
    idx: usize
}

impl Lexer {
    pub fn new(file_contents: String) -> Self {
        Self {
            file_contents,
            idx: 0
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut buf = vec![];
        let chars = self.file_contents
            .chars()
            .filter(|c| c.is_ascii());

        for chr in chars {
            self.idx += 1;

            match chr {
                '+' => buf.push(Token::Add),
                '-' => buf.push(Token::Sub),
                _ => return Err(LexerError::UnknownCharacter(chr))
            }
        }

        Ok(buf)
    }

    fn is_eof(&self) -> bool {
        self.idx >= self.file_contents.len() 
    }
}
