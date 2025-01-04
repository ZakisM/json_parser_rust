use crate::{
    ast::{JsonProperty, JsonValue},
    error::ExpectedTokenError,
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
        $self.expect_peek(Token::$variant)?;
    };
    ($self:expr, $variant:ident()) => {{
        $self.expect_peek(Token::$variant(Default::default()))?;

        let Token::$variant(value) = $self.current_token.clone() else {
            unreachable!();
        };

        value
    }};
}

macro_rules! expected_token_err {
    ($token:expr, $( $variant:ident )|+) => {
        return Err(ExpectedTokenError {
            expected: vec![$(Token::$variant),+],
            actual: $token.clone().into_owned(),
        })
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
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected: Token<'a>) -> Result<(), ExpectedTokenError> {
        if std::mem::discriminant(&self.peek_token) != std::mem::discriminant(&expected) {
            return Err(ExpectedTokenError {
                expected: vec![expected.clone().into_owned()],
                actual: self.peek_token.clone().into_owned(),
            });
        }

        self.next_token();

        Ok(())
    }

    fn parse_string(&self, literal: TokenLiteral<'a>) -> Result<JsonValue, ExpectedTokenError> {
        let s = String::from_utf8(literal.0.into_owned()).unwrap();

        Ok(JsonValue::String(s))
    }

    fn parse_number(&self, literal: TokenLiteral<'a>) -> Result<JsonValue, ExpectedTokenError> {
        let s = std::str::from_utf8(&literal.0).unwrap();
        let n = s.parse::<usize>().unwrap();

        Ok(JsonValue::Number(n))
    }

    fn parse_array(&mut self) -> Result<JsonValue, ExpectedTokenError> {
        expect_token!(self, LBracket);

        let mut items = Vec::new();

        loop {
            let value = self.parse_value()?;
            items.push(value);

            match self.peek_token {
                Token::Comma => self.next_token(),
                Token::RBracket => break,
                _ => {
                    expected_token_err!(self.peek_token, Comma | RBracket)
                }
            }
        }

        Ok(JsonValue::Array(items))
    }

    fn parse_value(&mut self) -> Result<JsonValue, ExpectedTokenError> {
        let value = match self.peek_token.clone() {
            Token::String(literal) => self.parse_string(literal)?,
            Token::Number(literal) => self.parse_number(literal)?,
            Token::True => JsonValue::Boolean(true),
            Token::False => JsonValue::Boolean(false),
            Token::Null => JsonValue::Null,
            Token::LBrace => self.parse_object()?,
            Token::LBracket => self.parse_array()?,
            _ => {
                return Err(ExpectedTokenError {
                    expected: vec![
                        Token::String(Default::default()),
                        Token::Number(Default::default()),
                        Token::True,
                        Token::False,
                        Token::Null,
                        Token::LBrace,
                        Token::LBracket,
                    ],
                    actual: self.peek_token.clone().into_owned(),
                });
            }
        };
        self.next_token();

        Ok(value)
    }

    fn parse_property(&mut self) -> Result<JsonProperty, ExpectedTokenError> {
        let key_token = expect_token!(self, String());
        let key = String::from_utf8(key_token.0.into_owned()).unwrap();

        expect_token!(self, Colon);

        let value = self.parse_value()?;

        Ok(JsonProperty::from((key, value)))
    }

    fn parse_object(&mut self) -> Result<JsonValue, ExpectedTokenError> {
        expect_token!(self, LBrace);

        let mut items = Vec::new();

        loop {
            let item = self.parse_property()?;
            items.push(item);

            match self.peek_token {
                Token::Comma => self.next_token(),
                Token::RBrace => break,
                _ => {
                    expected_token_err!(self.peek_token, Comma | RBrace)
                }
            }
        }

        Ok(JsonValue::Object(items))
    }

    fn parse(mut self) -> Result<JsonValue, ExpectedTokenError> {
        let result = self.parse_object()?;

        self.next_token();

        if !matches!(
            (&self.current_token, &self.peek_token),
            (Token::RBrace, Token::Eof)
        ) {
            expected_token_err!(self.current_token, Eof)
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
