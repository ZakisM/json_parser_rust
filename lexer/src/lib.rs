use std::borrow::Cow;

mod ast;
mod error;
mod parser;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct TokenLiteral<'a>(Cow<'a, [u8]>);

impl std::fmt::Display for TokenLiteral<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = std::str::from_utf8(&self.0).unwrap_or("Unknown data");
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token<'a> {
    // Values
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
    Illegal,
    Eof,
}

impl Token<'_> {
    fn into_owned(self) -> Token<'static> {
        match self {
            Token::String(token_literal) | Token::Number(token_literal) => {
                let inner = token_literal.0.into_owned();

                Token::String(TokenLiteral(Cow::Owned(inner)))
            }
            Token::True => Token::True,
            Token::False => Token::False,
            Token::Null => Token::Null,
            Token::LBrace => Token::LBrace,
            Token::RBrace => Token::RBrace,
            Token::LBracket => Token::LBracket,
            Token::RBracket => Token::RBracket,
            Token::Colon => Token::Colon,
            Token::Comma => Token::Comma,
            Token::Illegal => Token::Illegal,
            Token::Eof => Token::Eof,
        }
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Token::String(token_literal) | Token::Number(token_literal) => {
                &token_literal.to_string()
            }
            Token::True => "True",
            Token::False => "False",
            Token::Null => "Null",
            Token::LBrace => "LBrace",
            Token::RBrace => "RBrace",
            Token::LBracket => "LBracket",
            Token::RBracket => "RBracket",
            Token::Colon => "Colon",
            Token::Comma => "Comma",
            Token::Illegal => "Illegal",
            Token::Eof => "Eof",
        };

        write!(f, "{value}")
    }
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

    fn peek_char(&self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_position])
        }
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

            match self.ch {
                Some(b'"') | None => break,
                Some(b'\\') if self.peek_char() == Some(b'"') => self.read_char(),
                _ => continue,
            };
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

                return Token::Number(TokenLiteral(Cow::Borrowed(num)));
            }
            Some(b'"') => {
                let str = self.read_string();

                Token::String(TokenLiteral(Cow::Borrowed(str)))
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

    macro_rules! tok {
        ('{') => {
            Token::LBrace
        };
        ('}') => {
            Token::RBrace
        };
        ('[') => {
            Token::LBracket
        };
        (']') => {
            Token::RBracket
        };
        (':') => {
            Token::Colon
        };
        (',') => {
            Token::Comma
        };
        (true) => {
            Token::True
        };
        (false) => {
            Token::False
        };
        (null) => {
            Token::Null
        };
        (Eof) => {
            Token::Eof
        };
    }
    macro_rules! tok_str {
        ($str:literal) => {
            Token::String(TokenLiteral(Cow::Borrowed($str.as_bytes())))
        };
    }
    macro_rules! tok_num {
        ($num:literal) => {
            Token::Number(TokenLiteral(Cow::Borrowed(stringify!($num).as_bytes())))
        };
    }

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

        let expected_tokens = [
            tok!('{'),
            tok_str!("string"),
            tok!(':'),
            tok_str!("Hello, world!"),
            tok!(','),
            tok_str!("number"),
            tok!(':'),
            tok_num!(42),
            tok!(','),
            tok_str!("boolean"),
            tok!(':'),
            tok!(true),
            tok!(','),
            tok_str!("null"),
            tok!(':'),
            tok!(null),
            tok!(','),
            tok_str!("array"),
            tok!(':'),
            tok!('['),
            tok_num!(1),
            tok!(','),
            tok_num!(2),
            tok!(','),
            tok_num!(3),
            tok!(','),
            tok_num!(4),
            tok!(','),
            tok_str!("five"),
            tok!(','),
            tok!(true),
            tok!(']'),
            tok!(','),
            tok_str!("nested_object"),
            tok!(':'),
            tok!('{'),
            tok_str!("nested_string"),
            tok!(':'),
            tok_str!("This is a nested string"),
            tok!(','),
            tok_str!("nested_number"),
            tok!(':'),
            tok_num!(100),
            tok!(','),
            tok_str!("nested_array"),
            tok!(':'),
            tok!('['),
            tok_num!(10),
            tok!(','),
            tok_num!(20),
            tok!(','),
            tok_num!(30),
            tok!(']'),
            tok!(','),
            tok_str!("nested_boolean"),
            tok!(':'),
            tok!(false),
            tok!('}'),
            tok!(','),
            tok_str!("another_nested_object"),
            tok!(':'),
            tok!('{'),
            tok_str!("level1"),
            tok!(':'),
            tok!('{'),
            tok_str!("level2"),
            tok!(':'),
            tok!('{'),
            tok_str!("key"),
            tok!(':'),
            tok_str!("value"),
            tok!('}'),
            tok!('}'),
            tok!('}'),
            tok!('}'),
            tok!(Eof),
        ];

        for tok in expected_tokens {
            assert_eq!(lexer.next_token(), tok);
        }
    }

    #[test]
    fn parse_escaped() {
        let json = r#"{"key":"Hello, \"world!\""}"#;

        let mut lexer = Lexer::new(json.as_bytes());

        let expected_tokens = [
            tok!('{'),
            tok_str!("key"),
            tok!(':'),
            tok_str!(r#"Hello, \"world!\""#),
            tok!('}'),
        ];

        for tok in expected_tokens {
            assert_eq!(lexer.next_token(), tok);
        }
    }
}
