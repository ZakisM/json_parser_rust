use std::str::Chars;

use crate::{error::IllegalReason, illegal_number, illegal_string};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub origin: &'a str,
    pub start_column: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    Illegal(Option<IllegalReason>),
    Eof,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::Illegal(None)
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TokenKind::String => "STRING",
            TokenKind::Number => "NUMBER",
            TokenKind::True => "TRUE",
            TokenKind::False => "FALSE",
            TokenKind::Null => "NULL",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Illegal(reason) => match reason {
                Some(reason) => &format!("ILLEGAL ({reason})"),
                None => "ILLEGAL",
            },
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
    chars: Chars<'a>,
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

        while matches!(self.ch, Some(c) if c.is_ascii_lowercase()) {
            self.read_char();
        }

        &self.input[start_pos..self.position]
    }

    fn read_number(&mut self) -> &'a str {
        let start_pos = self.position;

        while matches!(self.ch, Some('0'..='9' | '.' | '-' | '+' | 'e' | 'E')) {
            self.read_char();
        }

        &self.input[start_pos..self.position]
    }

    fn is_legal_unicode(&mut self) -> Option<IllegalReason> {
        let start_pos = self.position;
        let start_column = self.column;

        for _ in 0..4 {
            if !matches!(self.ch, Some(c) if c.is_ascii_hexdigit()) {
                return illegal_string!(InvalidUnicode, start_column - 2);
            }

            self.read_char();
        }

        let codepoint = &self.input[start_pos..self.position];

        if !u32::from_str_radix(codepoint, 16).is_ok_and(|v| v <= 0x10FFFF) {
            return illegal_string!(InvalidUnicode, start_column - 2);
        }

        None
    }

    fn read_string(&mut self) -> (&'a str, Option<IllegalReason>) {
        self.read_char(); // consume opening double-quote

        let start_pos = self.position;
        let start_column = self.column;
        let mut illegal_reason = None;
        let mut has_closing_quote = false;

        while let Some(ch) = self.ch {
            match ch {
                '"' => {
                    self.read_char(); // consume closing double-quote
                    has_closing_quote = true;
                    break;
                }
                '\\' => {
                    self.read_char();

                    if matches!(
                        self.ch,
                        Some('"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't')
                    ) {
                        // If character is properly escaped then consume it
                        self.read_char();
                    } else if illegal_reason.is_none() {
                        // If current string is legal then do extra check
                        // to see if valid unicode otherwise must be an invalid
                        // escape character i.e \x \abc
                        if matches!(self.ch, Some('u')) {
                            self.read_char();
                            illegal_reason = self.is_legal_unicode();
                        } else {
                            illegal_reason = illegal_string!(InvalidEscape, self.column - 1);
                        }
                    }

                    continue;
                }
                '\t' if illegal_reason.is_none() => {
                    illegal_reason = illegal_string!(UnescapedTab, self.column);
                }
                '\n' if illegal_reason.is_none() => {
                    illegal_reason = illegal_string!(UnescapedNewLine, self.column);
                }
                _ => (),
            };

            self.read_char();
        }

        if has_closing_quote {
            (&self.input[start_pos..self.position - 1], illegal_reason)
        } else {
            (
                &self.input[start_pos..self.position],
                illegal_string!(MissingClosingQuote, start_column - 1),
            )
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        let start_column = self.column;

        let kind = match self.ch {
            Some('{') => TokenKind::LBrace,
            Some('}') => TokenKind::RBrace,
            Some('[') => TokenKind::LBracket,
            Some(']') => TokenKind::RBracket,
            Some(':') => TokenKind::Colon,
            Some(',') => TokenKind::Comma,
            Some('"') => {
                let (str, illegal_reason) = self.read_string();
                let kind = match illegal_reason {
                    Some(reason) => TokenKind::Illegal(Some(reason)),
                    None => TokenKind::String,
                };

                return Token {
                    kind,
                    origin: str,
                    start_column,
                };
            }
            Some('t' | 'f' | 'n') => {
                let ident = self.read_ident();

                let kind = match ident {
                    "true" => TokenKind::True,
                    "false" => TokenKind::False,
                    "null" => TokenKind::Null,
                    _ => TokenKind::Illegal(None),
                };

                return Token {
                    kind,
                    origin: ident,
                    start_column,
                };
            }
            Some('-' | '0'..='9') => {
                let num = self.read_number();

                let kind = match num.as_bytes() {
                    [b'0', b'0'..=b'9', ..] => illegal_number!(LeadingZero),
                    [b'0', b'e' | b'E', ..] => illegal_number!(MissingExponent),
                    [b'-', b'.', ..] => illegal_number!(InvalidFractionPart),
                    [.., b'.'] => illegal_number!(MissingFraction),
                    [.., b'-'] => illegal_number!(MinusMissingDigit),
                    [.., b'+'] => illegal_number!(MissingExponent),
                    bytes if bytes.windows(2).any(|w| w == b".e" || w == b".E") => {
                        illegal_number!(MissingFraction)
                    }
                    _ => TokenKind::Number,
                };

                return Token {
                    kind,
                    origin: num,
                    start_column,
                };
            }
            _ if self.position >= self.input.len() => {
                self.read_char();

                return Token {
                    kind: TokenKind::Eof,
                    start_column: start_column + 1,
                    ..Default::default()
                };
            }
            _ => TokenKind::Illegal(None),
        };

        let origin = &self.input[self.position..self.read_position];

        self.read_char();

        Token {
            kind,
            origin,
            start_column,
        }
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
    fn tokenize_escaped_1() {
        let json = r#"{"key":"Hello, \"world!\""}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_escaped_2() {
        let json = r#"{"key":"\""}"#;

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
    fn tokenize_valid_unicode_4() {
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

    #[test]
    fn tokenize_invalid_unicode_length_3() {
        let json = r#"{"key": "\uabc\u1234"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_unicode_length_4() {
        let json = r#"{"key": "\u1234\uabc"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_unicode_length_5() {
        let json = r#"{"key": "\ux\""}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_escape_1() {
        let json = r#"{"key": "\"}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_escape_2() {
        let json = r#"{"key": "\"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }

    #[test]
    fn tokenize_invalid_escape_3() {
        let json = r#"{"key": "\}"#;

        let lexer = Lexer::new(json);

        insta::assert_debug_snapshot!(&lexer.collect::<Vec<_>>());
    }
}
