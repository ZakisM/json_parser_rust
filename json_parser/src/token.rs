use std::{iter::Peekable, str::Chars};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub origin: &'a str,
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
    input: &'a str,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    pub row: usize,
    pub column: usize,
    ch: Option<char>,
    chars: Peekable<Chars<'a>>, // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars().peekable();

        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            row: 1,
            column: 0,
            ch: None,
            chars,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        match self.chars.next() {
            Some(ch) => {
                self.ch = Some(ch);
                self.column += 1;

                self.position = self.read_position;
                self.read_position += ch.len_utf8();
            }
            None => {
                self.read_position = self.input.len();
                self.ch = None;
            }
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                Some(' ' | '\t' | '\r') => (),
                Some('\n') => {
                    self.row += 1;
                    self.column = 0;
                }
                _ => break,
            }

            self.read_char();
        }
    }

    fn read_ident(&mut self) -> &'a str {
        let start_pos = self.position;

        while let Some(c) = self.ch {
            if !c.is_ascii_lowercase() {
                break;
            }

            self.read_char();
        }

        &self.input[start_pos..self.position]
    }

    fn read_number(&mut self) -> &'a str {
        let start_pos = self.position;

        loop {
            self.read_char();

            if !matches!(self.ch, Some('0'..='9' | '.' | '-' | '+' | 'e' | 'E')) {
                break;
            }
        }

        &self.input[start_pos..self.position]
    }

    fn read_string(&mut self) -> &'a str {
        let start_pos = self.position + 1;

        loop {
            self.read_char();

            match self.ch {
                Some('"') => {
                    self.read_char();
                    break;
                }
                Some('\\') if matches!(self.chars.peek(), Some('"' | '\\')) => self.read_char(),
                None => break,
                _ => continue,
            };
        }

        &self.input[start_pos..self.position - 1]
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        let kind = match self.ch {
            Some('{') => TokenKind::LBrace,
            Some('}') => TokenKind::RBrace,
            Some('[') => TokenKind::LBracket,
            Some(']') => TokenKind::RBracket,
            Some(':') => TokenKind::Colon,
            Some(',') => TokenKind::Comma,
            Some('"') => {
                return Token {
                    kind: TokenKind::String,
                    origin: self.read_string(),
                }
            }
            Some('t' | 'f' | 'n') => {
                let ident = self.read_ident();

                let kind = match ident {
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "null" => TokenKind::Null,
                    _ => TokenKind::Illegal,
                };

                return Token {
                    kind,
                    origin: ident,
                };
            }
            Some('-' | '0'..='9') => {
                let num = self.read_number();

                let kind = match num {
                    "-" => TokenKind::Illegal,
                    _ => TokenKind::Number,
                };

                return Token { kind, origin: num };
            }
            _ if self.read_position == self.input.len() => {
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
            origin: $string,
        }
    };
    (n $number:literal) => {
        Token {
            kind: TokenKind::Number,
            origin: stringify!($number),
        }
    };
    (true) => {
        Token {
            kind: TokenKind::True,
            origin: "true",
        }
    };
    (false) => {
        Token {
            kind: TokenKind::False,
            origin: "false",
        }
    };
    (null) => {
        Token {
            kind: TokenKind::Null,
            origin: "null",
        }
    };
    ('{') => {
        Token {
            kind: TokenKind::LBrace,
            origin: "{",
        }
    };
    ('}') => {
        Token {
            kind: TokenKind::RBrace,
            origin: "}",
        }
    };
    ('[') => {
        Token {
            kind: TokenKind::LBracket,
            origin: "[",
        }
    };
    (']') => {
        Token {
            kind: TokenKind::RBracket,
            origin: "]",
        }
    };
    (':') => {
        Token {
            kind: TokenKind::Colon,
            origin: ":",
        }
    };
    (',') => {
        Token {
            kind: TokenKind::Comma,
            origin: ",",
        }
    };
    (Illegal) => {
        Token {
            kind: TokenKind::Illegal,
            origin: "Illegal",
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
	"array": [1, 2, 3, 4eee, "five", true],
	"nested_object": {
		"nested_string": "This is a nested string",
		"nested_number": 100,
		"nested_array": [10, 3.21865081787e-6, 30],
		"nested_boolean": false
	},
	"another_nested_object": { "level1": { "level2": { "key": "value" } } }
}
"#;

        let mut lexer = Lexer::new(json);

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
                origin: "-42",
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
            Token {
                kind: TokenKind::Number,
                origin: "4eee",
            },
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
            Token {
                kind: TokenKind::Number,
                origin: "3.21865081787e-6",
            },
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

        let mut lexer = Lexer::new(json);

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

        let mut lexer = Lexer::new(json);

        let expected_tokens = [
            tok!('{'),
            tok!(s "number"),
            tok!(':'),
            Token {
                kind: TokenKind::Illegal,
                origin: "-",
            },
            tok!('}'),
        ];

        for tok in expected_tokens {
            assert_eq!(lexer.next_token(), tok);
        }
    }
}
