use core::panic;

use crate::{
    ast::{JsonItem, JsonValue},
    Lexer, Token, TokenLiteral,
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
}

macro_rules! expect_token {
    ($self:expr, $variant:ident) => {
        $self.expect_peek(&Token::$variant)
    };
    ($self:expr, $variant:ident()) => {
        $self.expect_peek(&Token::$variant(Default::default()))
    };
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(input),
            current_token: Token::Illegal,
            peek_token: Token::Illegal,
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected: &Token<'_>) -> bool {
        if std::mem::discriminant(&self.peek_token) == std::mem::discriminant(expected) {
            self.next_token();

            return true;
        }

        false
    }

    fn parse_val_string(&mut self, literal: TokenLiteral<'a>) -> Option<JsonValue<'a>> {
        let s = std::str::from_utf8(literal.0).unwrap();

        let value = JsonValue::String(s);

        Some(value)
    }

    fn parse_val_number(&mut self, literal: TokenLiteral<'a>) -> Option<JsonValue<'a>> {
        let s = std::str::from_utf8(literal.0).unwrap();
        let n = s.parse::<usize>().unwrap();

        let value = JsonValue::Number(n);

        Some(value)
    }

    fn parse_val_array(&mut self) -> Option<JsonValue<'a>> {
        let Token::LBracket = self.current_token else {
            panic!("must start with LBracket");
        };

        let mut items = Vec::new();

        loop {
            let value = match self.peek_token {
                Token::String(token_literal) => self
                    .parse_val_string(token_literal)
                    .expect("expected JsonString"),
                Token::Number(token_literal) => self
                    .parse_val_number(token_literal)
                    .expect("expected JsonNumber"),
                Token::True => JsonValue::Boolean(true),
                Token::False => JsonValue::Boolean(false),
                Token::Null => JsonValue::Null,
                Token::LBrace => {
                    self.next_token();
                    self.parse_object()
                }
                Token::LBracket => {
                    self.next_token();
                    self.parse_val_array().expect("expected JsonArray")
                }
                _ => panic!("unexpected token"),
            };
            self.next_token();

            items.push(value);

            if self.peek_token == Token::RBracket {
                break;
            }

            let Token::Comma = self.peek_token else {
                panic!("expected comma, found {:?}", self.peek_token);
            };
            self.next_token();
        }

        Some(JsonValue::Array(items))
    }

    fn parse_object(&mut self) -> JsonValue<'a> {
        let Token::LBrace = self.current_token else {
            panic!("must start with LBrace");
        };

        let mut items = Vec::new();

        loop {
            // Parse an item
            let Token::String(key) = self.peek_token else {
                panic!("expected string, found {:?}", self.peek_token);
            };
            self.next_token();

            let Token::Colon = self.peek_token else {
                panic!("expected colon, found {:?}", self.peek_token);
            };
            self.next_token();

            let value = match self.peek_token {
                Token::String(token_literal) => self
                    .parse_val_string(token_literal)
                    .expect("expected JsonString"),
                Token::Number(token_literal) => self
                    .parse_val_number(token_literal)
                    .expect("expected JsonNumber"),
                Token::True => JsonValue::Boolean(true),
                Token::False => JsonValue::Boolean(false),
                Token::Null => JsonValue::Null,
                Token::LBrace => {
                    self.next_token();
                    self.parse_object()
                }
                Token::LBracket => {
                    self.next_token();
                    self.parse_val_array().expect("expected JsonArray")
                }
                _ => panic!("unexpected token"),
            };
            self.next_token();

            items.push(JsonItem {
                key: std::str::from_utf8(key.0).unwrap(),
                value,
            });

            if self.peek_token == Token::RBrace {
                break;
            }

            let Token::Comma = self.peek_token else {
                panic!("expected comma, found {:?}", self.peek_token);
            };
            self.next_token();
        }

        JsonValue::Object(items)
    }

    fn parse(mut self) -> JsonValue<'a> {
        let res = self.parse_object();
        self.next_token();

        if self.current_token == Token::RBrace && self.peek_token != Token::Eof {
            panic!("someting wrong");
        }

        res
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
	"nested_object": {
		"nested_string": "This is a nested string",
		"nested_number": [100, 200, 300, [400, 500, [600, [700, {"secret": 12345}]]]]
	},
	"boolean": true
}
"#;

        let parser = Parser::new(json.as_bytes());

        dbg!(&parser.parse());
    }
}
