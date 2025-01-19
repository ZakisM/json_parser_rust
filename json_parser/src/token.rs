use std::str::Chars;

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
    chars: Chars<'a>, // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let chars = input.chars();

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
                self.ch = None;

                self.position = self.read_position;
                self.read_position += 1;
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

    fn is_legal_unicode(&mut self) -> bool {
        let start_pos = self.position;

        for _ in 0..4 {
            if !matches!(self.ch, Some(c) if c.is_ascii_hexdigit()) {
                break;
            }

            self.read_char();
        }

        let codepoint = &self.input[start_pos..self.position];

        if codepoint.len() != 4 {
            return false;
        }

        u32::from_str_radix(codepoint, 16).is_ok_and(|v| v <= 0x10FFFF)
    }

    fn is_legal_escaped_character(&mut self) -> bool {
        match self.ch {
            Some('"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't') => {
                self.read_char();
                true
            }
            Some('u') => {
                self.read_char();
                self.is_legal_unicode()
            }
            _ => false,
        }
    }

    fn read_string(&mut self) -> (&'a str, bool) {
        let start_pos = self.position;
        let mut legal = true;

        while let Some(ch) = self.ch {
            match ch {
                '"' => {
                    self.read_char();
                    break;
                }
                '\\' => {
                    self.read_char();
                    legal = self.is_legal_escaped_character();
                    continue;
                }
                '\t' => legal = false,
                _ => (),
            };

            self.read_char();
        }

        (&self.input[start_pos..self.position - 1], legal)
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
                self.read_char();

                let (str, legal) = self.read_string();

                return Token {
                    kind: if legal {
                        TokenKind::String
                    } else {
                        TokenKind::Illegal
                    },
                    origin: str,
                };
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

                let kind = match num.as_bytes() {
                    b"-" | [b'0', b'0'..=b'9', ..] => TokenKind::Illegal,
                    _ => TokenKind::Number,
                };

                return Token { kind, origin: num };
            }
            _ if self.position == self.input.len() => {
                self.read_char();

                return Token {
                    kind: TokenKind::Eof,
                    ..Default::default()
                };
            }
            _ => TokenKind::Illegal,
        };

        let origin = &self.input[self.position..self.read_position];

        self.read_char();

        Token { kind, origin }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position > self.input.len() {
            return None;
        }

        Some(self.next_token())
    }
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

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_escaped() {
        let json = r#"{"key":"Hello, \"world!\""}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_number() {
        let json = r#"{"number": -}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_valid_unicode_1() {
        let json = r#"{"key": "\u1234"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_valid_unicode_2() {
        let json = r#"{"key": "\u12345"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_valid_unicode_3() {
        let json = r#"{"key": "\udbcd"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_unicode() {
        let json = r#"{"key": "\uda00"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_unicode_length_1() {
        let json = r#"{"key": "\u"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_unicode_length_2() {
        let json = r#"{"key": "\u1"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }
}
