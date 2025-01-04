use crate::{
    ast::{JsonProperty, JsonValue},
    error::ExpectedTokenError,
    Lexer, Token, TokenKind,
};

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
            // TODO: Optimize
            current_token: Token {
                kind: TokenKind::Illegal,
                ..Default::default()
            },
            peek_token: Token {
                kind: TokenKind::Illegal,
                ..Default::default()
            },
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
            return Err(ExpectedTokenError {
                expected: vec![expected],
                actual: self.peek_token.kind.clone(),
            });
        }

        self.next_token();

        Ok(())
    }

    fn parse_string(&self, literal: &'a [u8]) -> Result<JsonValue, ExpectedTokenError> {
        // TODO: Cow
        let literal = String::from_utf8(literal.to_vec()).unwrap();

        Ok(JsonValue::String(literal))
    }

    fn parse_number(&self, literal: &'a [u8]) -> Result<JsonValue, ExpectedTokenError> {
        let s = std::str::from_utf8(literal).unwrap();
        let n = s.parse::<usize>().expect("literal must be a number");

        Ok(JsonValue::Number(n))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ExpectedTokenError> {
        self.expect_peek(TokenKind::LBracket)?;

        let mut items = Vec::new();

        loop {
            let value = self.parse_value()?;
            items.push(value);

            match self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBracket => break,
                _ => {
                    // expected_token_err!(self.peek_token, Comma | RBracket)
                    panic!()
                }
            }
        }

        Ok(JsonValue::Array(items))
    }

    fn parse_value(&mut self) -> Result<JsonValue, ExpectedTokenError> {
        let value = match self.peek_token.kind {
            TokenKind::String => self.parse_string(self.peek_token.origin)?,
            TokenKind::Number => self.parse_number(self.peek_token.origin)?,
            TokenKind::True => JsonValue::Boolean(true),
            TokenKind::False => JsonValue::Boolean(false),
            TokenKind::Null => JsonValue::Null,
            TokenKind::LBrace => self.parse_object()?,
            TokenKind::LBracket => self.parse_array()?,
            _ => {
                return Err(ExpectedTokenError {
                    expected: vec![
                        TokenKind::String,
                        TokenKind::Number,
                        TokenKind::True,
                        TokenKind::False,
                        TokenKind::Null,
                        TokenKind::LBrace,
                        TokenKind::LBracket,
                    ],
                    actual: self.peek_token.kind,
                });
            }
        };
        self.next_token();

        Ok(value)
    }

    fn parse_property(&mut self) -> Result<JsonProperty, ExpectedTokenError> {
        self.expect_peek(TokenKind::String)?;

        let key_token = self.current_token.origin;

        let key = String::from_utf8(key_token.to_vec()).unwrap();

        self.expect_peek(TokenKind::Colon)?;

        let value = self.parse_value()?;

        Ok(JsonProperty::from((key, value)))
    }

    fn parse_object(&mut self) -> Result<JsonValue, ExpectedTokenError> {
        self.expect_peek(TokenKind::LBrace)?;

        let mut items = Vec::new();

        loop {
            let item = self.parse_property()?;
            items.push(item);

            match self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBrace => break,
                _ => {
                    // expected_token_err!(self.peek_token, Comma | RBrace)
                    panic!()
                }
            }
        }

        Ok(JsonValue::Object(items))
    }

    fn parse(mut self) -> Result<JsonValue, ExpectedTokenError> {
        let result = self.parse_object()?;

        self.next_token();

        if !matches!(
            (&self.current_token.kind, &self.peek_token.kind),
            (TokenKind::RBrace, TokenKind::Eof)
        ) {
            // expected_token_err!(self.current_token, Eof)
            panic!()
        }

        Ok(result)
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

        assert_eq!(
            parser.parse(),
            Ok(JsonValue::Object(vec![
                JsonProperty::from((
                    "string".to_owned(),
                    JsonValue::String("Hello, world!".to_owned())
                )),
                JsonProperty::from(("number".to_owned(), JsonValue::Number(42))),
                JsonProperty::from((
                    "nested_object".to_owned(),
                    JsonValue::Object(vec![
                        JsonProperty::from((
                            "nested_string".to_owned(),
                            JsonValue::String("This is a nested string".to_owned())
                        )),
                        JsonProperty::from((
                            "nested_number".to_owned(),
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
                                                "secret".to_owned(),
                                                JsonValue::Number(12345)
                                            ))])
                                        ])
                                    ])
                                ])
                            ])
                        ))
                    ])
                )),
                JsonProperty::from(("boolean".to_owned(), JsonValue::Boolean(true)))
            ]))
        );
    }
}
