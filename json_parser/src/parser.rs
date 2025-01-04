use crate::{
    ast::{JsonProperty, JsonValue},
    error::ExpectedTokenError,
    token::{Lexer, Token, TokenKind},
};

macro_rules! expected_token_err {
    ($self:expr, $token:path) => {
        return Err(ExpectedTokenError {
            expected: vec![$token],
            actual: $self.kind,
        })
    };
    ($self:expr, $( $variant:ident )|+) => {
        return Err(ExpectedTokenError {
            expected: vec![$(TokenKind::$variant),+],
            actual: $self.kind,
        })
    };
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(input),
            current_token: Token::default(),
            peek_token: Token::default(),
        };

        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected: TokenKind) -> Result<(), ExpectedTokenError> {
        if self.peek_token.kind != expected {
            expected_token_err!(self.peek_token, expected)
        }

        self.next_token();

        Ok(())
    }

    // TODO: Result here?
    fn parse_string(&self, literal: &'a [u8]) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let s = std::str::from_utf8(literal).expect("literal must be a string");

        Ok(JsonValue::String(s))
    }

    // TODO: Result here?
    fn parse_number(&self, literal: &'a [u8]) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let s = std::str::from_utf8(literal).expect("literal must be a string");
        let n = s.parse::<usize>().expect("literal must be a number");

        Ok(JsonValue::Number(n))
    }

    fn parse_value(&mut self) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let value = match self.peek_token.kind {
            TokenKind::String => self.parse_string(self.peek_token.origin)?,
            TokenKind::Number => self.parse_number(self.peek_token.origin)?,
            TokenKind::True => JsonValue::Boolean(true),
            TokenKind::False => JsonValue::Boolean(false),
            TokenKind::Null => JsonValue::Null,
            TokenKind::LBrace => self.parse_object()?,
            TokenKind::LBracket => self.parse_array()?,
            _ => {
                expected_token_err!(
                    self.peek_token,
                    String | Number | True | False | Null | LBrace | LBracket
                )
            }
        };
        self.next_token();

        Ok(value)
    }

    fn parse_property(&mut self) -> Result<JsonProperty<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::String)?;

        let key = std::str::from_utf8(self.current_token.origin).expect("literal must be a string");

        self.expect_peek(TokenKind::Colon)?;

        let value = self.parse_value()?;

        Ok(JsonProperty::from((key, value)))
    }

    fn parse_array(&mut self) -> Result<JsonValue<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::LBracket)?;

        let mut items = Vec::new();

        loop {
            let value = self.parse_value()?;
            items.push(value);

            match self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBracket => break,
                _ => {
                    expected_token_err!(self.peek_token, Comma | RBracket)
                }
            }
        }

        Ok(JsonValue::Array(items))
    }

    fn parse_object(&mut self) -> Result<JsonValue<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::LBrace)?;

        let mut items = Vec::new();

        loop {
            let item = self.parse_property()?;
            items.push(item);

            match self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBrace => break,
                _ => {
                    expected_token_err!(self.peek_token, Comma | RBrace)
                }
            }
        }

        Ok(JsonValue::Object(items))
    }

    fn parse_root_object(mut self) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let result = self.parse_object()?;

        self.next_token();

        if !matches!(
            (&self.current_token.kind, &self.peek_token.kind),
            (TokenKind::RBrace, TokenKind::Eof)
        ) {
            expected_token_err!(self.current_token, TokenKind::Eof)
        }

        Ok(result)
    }

    fn parse_root_array(mut self) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let result = self.parse_array()?;

        self.next_token();

        if !matches!(
            (&self.current_token.kind, &self.peek_token.kind),
            (TokenKind::RBracket, TokenKind::Eof)
        ) {
            expected_token_err!(self.current_token, TokenKind::Eof)
        }

        Ok(result)
    }

    fn parse(mut self) -> Result<JsonValue<'a>, ExpectedTokenError> {
        match self.peek_token.kind {
            TokenKind::LBrace => self.parse_root_object(),
            TokenKind::LBracket => self.parse_root_array(),
            _ => expected_token_err!(self.current_token, LBrace | LBracket),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_top_level_object() {
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

        assert_eq!(
            parser.parse(),
            Ok(JsonValue::Object(vec![
                JsonProperty::from(("string", JsonValue::String("Hello, world!"))),
                JsonProperty::from(("number", JsonValue::Number(42))),
                JsonProperty::from((
                    "nested_object",
                    JsonValue::Object(vec![
                        JsonProperty::from((
                            "nested_string",
                            JsonValue::String("This is a nested string")
                        )),
                        JsonProperty::from((
                            "nested_number",
                            JsonValue::Array(vec![
                                JsonValue::Number(100),
                                JsonValue::Number(200),
                                JsonValue::Number(300),
                                JsonValue::Array(vec![
                                    JsonValue::Number(400),
                                    JsonValue::Number(500),
                                    JsonValue::Array(vec![
                                        JsonValue::Number(600),
                                        JsonValue::Array(vec![
                                            JsonValue::Number(700),
                                            JsonValue::Object(vec![JsonProperty::from((
                                                "secret",
                                                JsonValue::Number(12345)
                                            ))])
                                        ])
                                    ])
                                ])
                            ])
                        ))
                    ])
                )),
                JsonProperty::from(("boolean", JsonValue::Boolean(true)))
            ]))
        );
    }

    #[test]
    fn parse_top_level_array() {
        let json = r#"
[
    {
        "one": 1,
        "two": 2
    }
]
"#;

        let parser = Parser::new(json.as_bytes());

        assert_eq!(
            parser.parse(),
            Ok(JsonValue::Array(vec![JsonValue::Object(vec![
                JsonProperty::from(("one", JsonValue::Number(1))),
                JsonProperty::from(("two", JsonValue::Number(2)))
            ])]))
        );
    }
}
