use bumpalo::{collections::Vec, Bump};

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
    pub fn new(input: &'a [u8]) -> Self {
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
        // let s = std::str::from_utf8(literal).expect("literal must be a string");
        let s = unsafe { std::str::from_utf8_unchecked(literal) };

        Ok(JsonValue::String(s))
    }

    // TODO: Result here?
    fn parse_number(&self, literal: &'a [u8]) -> Result<JsonValue<'a>, ExpectedTokenError> {
        // let s = std::str::from_utf8(literal).expect("literal must be a string");
        let s = unsafe { std::str::from_utf8_unchecked(literal) };
        dbg!(&s);
        let n = s.parse::<isize>().expect("literal must be a number");

        Ok(JsonValue::Number(n))
    }

    fn parse_value(&mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let value = match &self.peek_token.kind {
            TokenKind::String => self.parse_string(self.peek_token.origin)?,
            TokenKind::Number => self.parse_number(self.peek_token.origin)?,
            TokenKind::True => JsonValue::Boolean(true),
            TokenKind::False => JsonValue::Boolean(false),
            TokenKind::Null => JsonValue::Null,
            TokenKind::LBrace => self.parse_object(bump)?,
            TokenKind::LBracket => self.parse_array(bump)?,
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

    fn parse_property(&mut self, bump: &'a Bump) -> Result<JsonProperty<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::String)?;

        // let key = std::str::from_utf8(self.current_token.origin).expect("literal must be a string");
        let key = unsafe { std::str::from_utf8_unchecked(self.current_token.origin) };

        self.expect_peek(TokenKind::Colon)?;

        let value = self.parse_value(bump)?;

        Ok(JsonProperty::from((key, value)))
    }

    fn parse_array(&mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::LBracket)?;

        if self.peek_token.kind == TokenKind::RBracket {
            return Ok(JsonValue::Array(Vec::new_in(bump)));
        }

        let mut items = Vec::with_capacity_in(4950000, bump);

        loop {
            let value = self.parse_value(bump)?;
            items.push(value);

            match &self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBracket => break,
                _ => {
                    expected_token_err!(self.peek_token, Comma | RBracket)
                }
            }
        }

        Ok(JsonValue::Array(items))
    }

    fn parse_object(&mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::LBrace)?;

        if self.peek_token.kind == TokenKind::RBrace {
            return Ok(JsonValue::Object(Vec::new_in(bump)));
        }

        let mut items = Vec::with_capacity_in(5, bump);

        loop {
            let item = self.parse_property(bump)?;
            items.push(item);

            match &self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBrace => break,
                _ => {
                    expected_token_err!(self.peek_token, Comma | RBrace)
                }
            }
        }

        Ok(JsonValue::Object(items))
    }

    fn parse_root_object(mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let result = self.parse_object(bump)?;

        self.next_token();

        if !matches!(
            (&self.current_token.kind, &self.peek_token.kind),
            (TokenKind::RBrace, TokenKind::Eof)
        ) {
            expected_token_err!(self.current_token, TokenKind::Eof)
        }

        Ok(result)
    }

    fn parse_root_array(mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let result = self.parse_array(bump)?;

        self.next_token();

        if !matches!(
            (&self.current_token.kind, &self.peek_token.kind),
            (TokenKind::RBracket, TokenKind::Eof)
        ) {
            expected_token_err!(self.current_token, TokenKind::Eof)
        }

        Ok(result)
    }

    pub fn parse(self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        match self.peek_token.kind {
            TokenKind::LBrace => self.parse_root_object(bump),
            TokenKind::LBracket => self.parse_root_array(bump),
            _ => expected_token_err!(self.current_token, LBrace | LBracket),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bumpalo::vec;

    #[test]
    fn parse_top_level_object() {
        let json = r#"
{
	"string": "Hello, world!",
	"number": -42,
	"nested_object": {
		"nested_string": "This is a nested string",
		"nested_number": [100, 200, 300, [400, -500, [600, [700, {"secret": 12345}]]]]
	},
	"boolean": true,
	"nested_deep_empty_array": [[[]], {}],
	"nested_empties": {
	    "empty_object": {},
	    "empty_array": []
	}
}
"#;

        let bump = Bump::new();
        let parser = Parser::new(json.as_bytes());

        assert_eq!(
            parser.parse(&bump),
            Ok(JsonValue::Object(vec![in &bump;
                JsonProperty::from(("string", JsonValue::String("Hello, world!"))),
                JsonProperty::from(("number", JsonValue::Number(-42))),
                JsonProperty::from((
                    "nested_object",
                    JsonValue::Object(vec![
                        in &bump;
                        JsonProperty::from((
                            "nested_string",
                            JsonValue::String("This is a nested string")
                        )),
                        JsonProperty::from((
                            "nested_number",
                            JsonValue::Array(vec![
                                in &bump;
                                JsonValue::Number(100),
                                JsonValue::Number(200),
                                JsonValue::Number(300),
                                JsonValue::Array(vec![
                                    in &bump;
                                    JsonValue::Number(400),
                                    JsonValue::Number(-500),
                                    JsonValue::Array(vec![
                                        in &bump;
                                        JsonValue::Number(600),
                                        JsonValue::Array(vec![
                                            in &bump;
                                            JsonValue::Number(700),
                                            JsonValue::Object(vec![
                                                in &bump;
                                                JsonProperty::from(("secret", JsonValue::Number(12345)
                                            ))])
                                        ])
                                    ])
                                ])
                            ])
                        ))
                    ])
                )),
                JsonProperty::from(("boolean", JsonValue::Boolean(true))),
                JsonProperty::from(("nested_deep_empty_array", JsonValue::Array(vec![
                    in &bump;
                    JsonValue::Array(vec![
                        in &bump;
                        JsonValue::Array(vec![
                            in &bump;
                        ])
                    ]),
                    JsonValue::Object(vec![
                        in &bump;
                    ])
                ]))),
                JsonProperty::from(("nested_empties", JsonValue::Object(vec![
                    in &bump;
                    JsonProperty::from(("empty_object", JsonValue::Object(vec![
                        in &bump;
                    ]))),
                    JsonProperty::from(("empty_array", JsonValue::Array(vec![
                        in &bump;
                    ])))
                ]))),
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

        let bump = Bump::new();
        let parser = Parser::new(json.as_bytes());

        assert_eq!(
            parser.parse(&bump),
            Ok(JsonValue::Array(vec![
                in &bump;
                JsonValue::Object(vec![
                    in &bump;
                    JsonProperty::from(("one", JsonValue::Number(1))),
                    JsonProperty::from(("two", JsonValue::Number(2)))
            ])]))
        );
    }
}
