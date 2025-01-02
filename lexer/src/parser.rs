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

    fn parse_string(&mut self, literal: TokenLiteral<'a>) -> Option<JsonValue<'a>> {
        let s = std::str::from_utf8(literal.0).unwrap();

        let value = JsonValue::String(s);

        Some(value)
    }

    fn parse_number(&mut self, literal: TokenLiteral<'a>) -> Option<JsonValue<'a>> {
        let s = std::str::from_utf8(literal.0).unwrap();
        let n = s.parse::<usize>().unwrap();

        let value = JsonValue::Number(n);

        Some(value)
    }

    fn parse_item(&mut self) -> Option<JsonItem<'a>> {
        let Token::String(key) = self.peek_token else {
            return None;
        };
        self.next_token();

        let Token::Colon = self.peek_token else {
            return None;
        };
        self.next_token();

        match self.current_token {
            Token::String(token_literal) => self.parse_string(token_literal)?,
            Token::Number(token_literal) => self.parse_number(token_literal)?,
            Token::True => JsonValue::Boolean(true),
            Token::False => JsonValue::Boolean(false),
            Token::Null => JsonValue::Null,
            Token::LBrace => todo!(),
            Token::RBrace => todo!(),
            Token::LBracket => todo!(),
            Token::RBracket => todo!(),
            Token::Colon => todo!(),
            Token::Comma => todo!(),
            _ => panic!("unhandled"),
        };

        None
    }

    fn parse_object(&mut self) -> Option<JsonItem<'a>> {
        match self.current_token {
            Token::LBrace => {
                let Token::String(_) = self.peek_token else {
                    return None;
                };

                let item = self.parse_item()?;

                Some(item)
            }
            _ => None,
        }
    }

    fn parse(mut self) -> JsonValue<'a> {
        let mut items = Vec::new();

        while self.current_token != Token::Eof {
            if let Some(item) = self.parse_object() {
                items.push(item);
            }

            self.next_token();
        }

        JsonValue::Object(items)
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
}
"#;

        let parser = Parser::new(json.as_bytes());

        dbg!(&parser.parse());
    }
}
