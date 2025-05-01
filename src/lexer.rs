use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident,
    Integer,
    Illegal,
    Eof,
    Int,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input,
        };
        lex.read_char();

        return lex;
    }

    pub fn next_token(&mut self) -> Result<Token> {
        if self.read_position >= self.input.len() {
            return Ok(Token::Eof);
        }

        let tok = match self.ch {
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            b'=' => Token::Assign,
            0 => Token::Eof,
            _ => todo!("need to implement this..."),
        };

        self.read_char();
        return Ok(tok);
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(Token::Eof) => {
                    tokens.push(Token::Eof);
                    break;
                }
                Ok(tok) => tokens.push(tok),
                Err(_) => break, // you could also collect errors
            }
        }

        tokens
    }
}

#[cfg(test)]
mod test {

    use super::{Lexer, Token};
    use anyhow::Result;

    #[test]
    fn get_next_token() -> Result<()> {
        let input = String::from("=+(){},;");
        let mut lexer = Lexer::new(input);

        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }
}
