#[derive(Debug, PartialEq, Eq)]
struct TokenLiteral<'a>(&'a [u8]);

#[derive(Debug, PartialEq, Eq)]
enum Token<'a> {
    Illegal,
    Eof,
    // Types
    String(TokenLiteral<'a>),
    Number(TokenLiteral<'a>),
    True,
    False,
    Null,
    // Delimiters
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
}

#[derive(Debug)]
struct Lexer<'a> {
    input: &'a [u8],
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let mut lexer = Self {
            input,
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
        while matches!(self.ch, Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.read_char();
        }
    }

    fn read_ident(&mut self) -> &'a [u8] {
        let start_pos = self.position;

        while let Some(c) = self.ch {
            if !c.is_ascii_alphabetic() {
                break;
            }

            self.read_char();
        }

        &self.input[start_pos..self.position]
    }

    fn read_number(&mut self) -> &'a [u8] {
        let start_pos = self.position;

        while let Some(c) = self.ch {
            if !c.is_ascii_digit() {
                break;
            }

            self.read_char();
        }

        &self.input[start_pos..self.position]
    }

    fn read_string(&mut self) -> &'a [u8] {
        let start_pos = self.position + 1;

        loop {
            self.read_char();

            if self.ch == Some(b'"') || self.ch.is_none() {
                break;
            }
        }

        &self.input[start_pos..self.position]
    }

    fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        let token = match self.ch {
            Some(b'{') => Token::LBrace,
            Some(b'}') => Token::RBrace,
            Some(b'[') => Token::LBracket,
            Some(b']') => Token::RBracket,
            Some(b'"') => {
                let str = self.read_string();

                Token::String(TokenLiteral(str))
            }
            Some(b':') => Token::Colon,
            Some(b',') => Token::Comma,
            Some(other) if other.is_ascii_alphabetic() => {
                let ident = self.read_ident();

                return match ident {
                    b"true" => Token::True,
                    b"false" => Token::False,
                    b"null" => Token::Null,
                    _ => Token::Illegal,
                };
            }
            Some(other) if other.is_ascii_digit() => {
                let num = self.read_number();

                return Token::Number(TokenLiteral(num));
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
        let json = r#"
{
	"string": "Hello, world!",
	"number": 42,
	"boolean": true,
	"null": null,
	"array": [1, 2, 3, 4, "five", true],
	"nested_object": {
		"nested_string": "This is a nested string",
		"nested_number": 100,
		"nested_array": [10, 20, 30],
		"nested_boolean": false
	},
	"another_nested_object": { "level1": { "level2": { "key": "value" } } }
}
"#;

        let mut lexer = Lexer::new(json.as_bytes());

        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::String(TokenLiteral(b"string")));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(
            lexer.next_token(),
            Token::String(TokenLiteral(b"Hello, world!"))
        );
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::String(TokenLiteral(b"number")));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Number(TokenLiteral(b"42")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::String(TokenLiteral(b"boolean")));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::True);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::String(TokenLiteral(b"null")));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::Null);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::String(TokenLiteral(b"array")));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::LBracket);
        assert_eq!(lexer.next_token(), Token::Number(TokenLiteral(b"1")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(TokenLiteral(b"2")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(TokenLiteral(b"3")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Number(TokenLiteral(b"4")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::String(TokenLiteral(b"five")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::True);
        assert_eq!(lexer.next_token(), Token::RBracket);
    }
}
