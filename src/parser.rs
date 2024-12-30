#[derive(Debug)]
enum Token {
    Illegal,
    Eof,
    // Identifiers
    Ident,
    Int,
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

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some(b'{') => Token::LBrace,
            _ => Token::Illegal,
        };

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_simple() {
        let json = r#"{"key": "value"}"#;
    }
}
