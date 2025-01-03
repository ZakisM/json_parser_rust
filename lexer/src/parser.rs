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

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected: &Token<'_>) -> bool {
        if std::mem::discriminant(&self.peek_token) == std::mem::discriminant(expected) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn parse_string(&mut self, literal: TokenLiteral<'a>) -> Option<JsonValue<'a>> {
        let s = std::str::from_utf8(literal.0).unwrap();

        Some(JsonValue::String(s))
    }

    fn parse_number(&mut self, literal: TokenLiteral<'a>) -> Option<JsonValue<'a>> {
        let s = std::str::from_utf8(literal.0).unwrap();
        let n = s.parse::<usize>().unwrap();

        Some(JsonValue::Number(n))
    }

    fn parse_array(&mut self) -> Option<JsonValue<'a>> {
        if !expect_token!(self, LBracket) {
            panic!("expected LBracket found {:?}", self.peek_token);
        }

        let mut items = Vec::new();

        loop {
            let value = self.parse_value();
            items.push(value);

            if self.peek_token == Token::RBracket {
                break;
            }

            if !expect_token!(self, Comma) {
                panic!("expected comma, found {:?}", self.peek_token);
            }
        }

        Some(JsonValue::Array(items))
    }

    fn parse_value(&mut self) -> JsonValue<'a> {
        let value = match self.peek_token {
            Token::String(literal) => self.parse_string(literal).expect("expected JsonString"),
            Token::Number(literal) => self.parse_number(literal).expect("expected JsonNumber"),
            Token::True => JsonValue::Boolean(true),
            Token::False => JsonValue::Boolean(false),
            Token::Null => JsonValue::Null,
            Token::LBrace => self.parse_object(),
            Token::LBracket => self.parse_array().expect("expected JsonArray"),
            _ => panic!("unexpected token"),
        };
        self.next_token();

        value
    }

    fn parse_item(&mut self) -> JsonItem<'a> {
        // Parse an item
        let Token::String(key) = self.peek_token else {
            panic!("expected string, found {:?}", self.peek_token);
        };
        self.next_token();

        if !expect_token!(self, Colon) {
            panic!("expected colon, found {:?}", self.peek_token);
        }

        let key = std::str::from_utf8(key.0).unwrap();
        let value = self.parse_value();

        JsonItem::from((key, value))
    }

    fn parse_object(&mut self) -> JsonValue<'a> {
        if !expect_token!(self, LBrace) {
            panic!("expected LBrace found {:?}", self.peek_token);
        }

        let mut items = Vec::new();

        loop {
            let item = self.parse_item();
            items.push(item);

            if self.peek_token == Token::RBrace {
                break;
            }

            if !expect_token!(self, Comma) {
                panic!("expected comma, found {:?}", self.peek_token);
            }
        }

        JsonValue::Object(items)
    }

    fn parse(mut self) -> JsonValue<'a> {
        let result = self.parse_object();
        self.next_token();

        if !matches!(
            (self.current_token, self.peek_token),
            (Token::RBrace, Token::Eof)
        ) {
            panic!("someting wrong");
        }

        result
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
