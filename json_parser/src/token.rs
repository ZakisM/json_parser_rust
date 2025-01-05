#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub origin: &'a [u8],
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Values
    String,
    Number,
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
    #[default]
    Illegal,
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TokenKind::String => "string",
            TokenKind::Number => "number",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Null => "null",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Illegal => "Not allowed",
            TokenKind::Eof => "End of file",
        };

        write!(f, "{value}")
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a [u8],
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    pub row: usize,
    pub column: usize,
    ch: Option<u8>, // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            row: 1,
            column: 0,
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
            self.column += 1;
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
        loop {
            match self.ch {
                Some(b' ' | b'\t' | b'\r') => (),
                Some(b'\n') => {
                    self.row += 1;
                    self.column = 0;
                }
                _ => break,
            }

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

        loop {
            self.read_char();

            match self.ch {
                Some(c) if !c.is_ascii_digit() => break,
                _ => continue,
            }
        }

        &self.input[start_pos..self.position]
    }

    fn read_string(&mut self) -> &'a [u8] {
        let start_pos = self.position + 1;

        loop {
            self.read_char();

            match self.ch {
                Some(b'"') | None => {
                    self.read_char();
                    break;
                }
                Some(b'\\') if self.peek_char() == Some(b'"') => self.read_char(),
                _ => continue,
            };
        }

        &self.input[start_pos..self.position - 1]
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        let kind = match self.ch {
            Some(b'{') => TokenKind::LBrace,
            Some(b'}') => TokenKind::RBrace,
            Some(b'[') => TokenKind::LBracket,
            Some(b']') => TokenKind::RBracket,
            Some(b':') => TokenKind::Colon,
            Some(b',') => TokenKind::Comma,
            Some(b'"') => {
                return Token {
                    kind: TokenKind::String,
                    origin: self.read_string(),
                }
            }
            Some(other) if other.is_ascii_alphabetic() => {
                let ident = self.read_ident();

                let kind = match ident {
                    b"true" => TokenKind::True,
                    b"false" => TokenKind::False,
                    b"null" => TokenKind::Null,
                    _ => TokenKind::Illegal,
                };

                return Token {
                    kind,
                    origin: ident,
                };
            }
            Some(other) if other == b'-' || other.is_ascii_digit() => {
                let num = self.read_number();

                let kind = match num {
                    b"-" => TokenKind::Illegal,
                    _ => TokenKind::Number,
                };

                return Token { kind, origin: num };
            }
            _ if self.read_position > self.input.len() => {
                return Token {
                    kind: TokenKind::Eof,
                    ..Default::default()
                }
            }
            _ => TokenKind::Illegal,
        };

        let origin = &self.input[self.position..self.read_position];

        self.read_char();

        Token { kind, origin }
    }
}

#[macro_export]
macro_rules! tok {
    (s $string:literal) => {
        Token {
            kind: TokenKind::String,
            origin: $string.as_bytes(),
        }
    };
    (n $number:literal) => {
        Token {
            kind: TokenKind::Number,
            origin: stringify!($number).as_bytes(),
        }
    };
    (true) => {
        Token {
            kind: TokenKind::True,
            origin: b"true",
        }
    };
    (false) => {
        Token {
            kind: TokenKind::False,
            origin: b"false",
        }
    };
    (null) => {
        Token {
            kind: TokenKind::Null,
            origin: b"null",
        }
    };
    ('{') => {
        Token {
            kind: TokenKind::LBrace,
            origin: b"{",
        }
    };
    ('}') => {
        Token {
            kind: TokenKind::RBrace,
            origin: b"}",
        }
    };
    ('[') => {
        Token {
            kind: TokenKind::LBracket,
            origin: b"[",
        }
    };
    (']') => {
        Token {
            kind: TokenKind::RBracket,
            origin: b"]",
        }
    };
    (':') => {
        Token {
            kind: TokenKind::Colon,
            origin: b":",
        }
    };
    (',') => {
        Token {
            kind: TokenKind::Comma,
            origin: b",",
        }
    };
    (Illegal) => {
        Token {
            kind: TokenKind::Illegal,
            origin: b"Illegal",
        }
    };
    (Eof) => {
        Token {
            kind: TokenKind::Eof,
            ..Default::default()
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        let json = r#"
{
	"string": "Hello, world!",
	"number": -42,
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
            tok!(s "string"),
            tok!(':'),
            tok!(s "Hello, world!"),
            tok!(','),
            tok!(s "number"),
            tok!(':'),
            Token {
                kind: TokenKind::Number,
                origin: b"-42",
            },
            tok!(','),
            tok!(s "boolean"),
            tok!(':'),
            tok!(true),
            tok!(','),
            tok!(s "null"),
            tok!(':'),
            tok!(null),
            tok!(','),
            tok!(s "array"),
            tok!(':'),
            tok!('['),
            tok!(n 1),
            tok!(','),
            tok!(n 2),
            tok!(','),
            tok!(n 3),
            tok!(','),
            tok!(n 4),
            tok!(','),
            tok!(s "five"),
            tok!(','),
            tok!(true),
            tok!(']'),
            tok!(','),
            tok!(s "nested_object"),
            tok!(':'),
            tok!('{'),
            tok!(s "nested_string"),
            tok!(':'),
            tok!(s "This is a nested string"),
            tok!(','),
            tok!(s "nested_number"),
            tok!(':'),
            tok!(n 100),
            tok!(','),
            tok!(s "nested_array"),
            tok!(':'),
            tok!('['),
            tok!(n 10),
            tok!(','),
            tok!(n 20),
            tok!(','),
            tok!(n 30),
            tok!(']'),
            tok!(','),
            tok!(s "nested_boolean"),
            tok!(':'),
            tok!(false),
            tok!('}'),
            tok!(','),
            tok!(s "another_nested_object"),
            tok!(':'),
            tok!('{'),
            tok!(s "level1"),
            tok!(':'),
            tok!('{'),
            tok!(s "level2"),
            tok!(':'),
            tok!('{'),
            tok!(s "key"),
            tok!(':'),
            tok!(s "value"),
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
    fn tokenize_escaped() {
        let json = r#"{"key":"Hello, \"world!\""}"#;

        let mut lexer = Lexer::new(json.as_bytes());

        let expected_tokens = [
            tok!('{'),
            tok!(s "key"),
            tok!(':'),
            tok!(s r#"Hello, \"world!\""#),
            tok!('}'),
        ];

        for tok in expected_tokens {
            assert_eq!(lexer.next_token(), tok);
        }
    }

    #[test]
    fn tokenize_invalid_number() {
        let json = r#"{"number": -}"#;

        let mut lexer = Lexer::new(json.as_bytes());

        let expected_tokens = [
            tok!('{'),
            tok!(s "number"),
            tok!(':'),
            Token {
                kind: TokenKind::Illegal,
                origin: b"-",
            },
            tok!('}'),
        ];

        for tok in expected_tokens {
            assert_eq!(lexer.next_token(), tok);
        }
    }
}
