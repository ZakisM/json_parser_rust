#[derive(Debug, PartialEq, Eq)]
struct TokenLiteral<'a>(&'a [u8]);

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Illegal,
    Eof,
    // Types
    String(TokenLiteral<'a>),
    Number(TokenLiteral<'a>),
    // Delimiters
    LBrace,
    RBrace,
    DoubleQuote,
    Colon,
    Comma,
}

#[derive(Debug)]
struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, Some(b' ' | b'\n' | b'\r')) {
            self.read_char();
        }
    }

    fn read_string(&mut self) -> &[u8] {
        let start_pos = self.position;

        while let Some(c) = self.ch {
            if !c.is_ascii_alphabetic() {
                break;
            }

            self.read_char();
        }

        &self.input[start_pos..self.position]
    }

    fn read_number(&mut self) -> &[u8] {
        let start_pos = self.position;

        while let Some(c) = self.ch {
            if !c.is_ascii_digit() {
                break;
            }

            self.read_char();
        }

        &self.input[start_pos..self.position]
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some(b'{') => Token::LBrace,
            Some(b'}') => Token::RBrace,
            Some(b'"') => Token::DoubleQuote,
            Some(b':') => Token::Colon,
            Some(b',') => Token::Comma,
            Some(other) if other.is_ascii_alphabetic() => {
                let s = self.read_string();

                return Token::String(TokenLiteral(s));
            }
            Some(other) if other.is_ascii_digit() => {
                let s = self.read_number();

                return Token::Number(TokenLiteral(s));
            }
            _ if self.read_position > self.input.len() => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let json = r#"{"key": 4}"#;

        let mut lexer = Lexer::new(json.to_owned());

        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::DoubleQuote);
        assert_eq!(lexer.next_token(), Token::String(TokenLiteral(b"key")));
        assert_eq!(lexer.next_token(), Token::DoubleQuote);
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Number(TokenLiteral(b"4")));
        assert_eq!(lexer.next_token(), Token::RBrace);
    }
}
