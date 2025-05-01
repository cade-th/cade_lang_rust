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
    pub fn lex(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(Token::Eof) => {
                    tokens.push(Token::Eof);
                    break;
                }
                Ok(tok) => tokens.push(tok),
                Err(e) => return Err(e), // you could also collect errors
            }
        }

        Ok(tokens)
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

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, b' ' | b'\n' | b'\r' | b'\t') {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

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
            _ => return Err(format!("unexpected char: {}", self.ch as char)),
        };

        self.read_char();
        return Ok(tok);
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn get_all_tokens() -> Result<(), String> {
        let input = "=+(){},;".to_string();
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        let tokens = lexer.lex();

        if tokens != expected_tokens {
            return Err("Token mismatch!".to_string());
        }

        Ok(())
    }
}
