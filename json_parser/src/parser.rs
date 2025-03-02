use bumpalo::{Bump, collections::Vec};

use crate::{
    ast::{JsonProperty, JsonValue},
    error::ExpectedTokenError,
    token::{IllegalNumber, IllegalReason, Lexer, Token, TokenKind},
};

macro_rules! expected_token_err {
    ($actual_token:expr, $row:expr, $column:expr, $expected_token:path) => {
        return Err(ExpectedTokenError::new(
            vec![$expected_token],
            $actual_token.kind.clone(),
            ($actual_token.origin).to_owned(),
            $row,
            $column,
        ))
    };
    ($actual_token:expr, $row:expr, $column:expr, $( $variant:ident )|+) => {
        return Err(ExpectedTokenError::new(
            vec![$(TokenKind::$variant),+],
            $actual_token.kind.clone(),
            ($actual_token.origin).to_owned(),
            $row,
            $column,
        ))
    };
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(input),
            current_token: Token::default(),
            peek_token: Token::default(),
        };

        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, expected: TokenKind) -> Result<(), ExpectedTokenError> {
        if self.peek_token.kind != expected {
            expected_token_err!(
                self.peek_token,
                self.lexer.row,
                self.peek_token.start_column,
                expected
            )
        }

        self.next_token();

        Ok(())
    }

    fn parse_string(&self, literal: &'a str) -> Result<JsonValue<'a>, ExpectedTokenError> {
        Ok(JsonValue::String(literal))
    }

    fn parse_number(&self, literal: &'a str) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let n = literal.parse::<f64>().map_err(|e| {
            ExpectedTokenError::new(
                vec![TokenKind::Number],
                TokenKind::Illegal(Some(IllegalReason::Number(IllegalNumber::ParseFloatError(
                    e,
                )))),
                literal.to_owned(),
                self.lexer.row,
                self.peek_token.start_column,
            )
        })?;

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
                    self.lexer.row,
                    self.peek_token.start_column,
                    String | Number | Null | LBrace | LBracket | True | False
                )
            }
        };
        self.next_token();

        Ok(value)
    }

    fn parse_property(&mut self, bump: &'a Bump) -> Result<JsonProperty<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::String)?;

        let key = self.current_token.origin;

        self.expect_peek(TokenKind::Colon)?;

        let value = self.parse_value(bump)?;

        Ok(JsonProperty::from((key, value)))
    }

    fn parse_array(&mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        self.expect_peek(TokenKind::LBracket)?;

        if self.peek_token.kind == TokenKind::RBracket {
            return Ok(JsonValue::Array(Vec::new_in(bump)));
        }

        let mut items = Vec::with_capacity_in(8, bump);

        loop {
            let value = self.parse_value(bump)?;
            items.push(value);

            match &self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBracket => break,
                _ => {
                    expected_token_err!(
                        self.peek_token,
                        self.lexer.row,
                        self.peek_token.start_column,
                        Comma | RBracket
                    )
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

        let mut items = Vec::with_capacity_in(8, bump);

        loop {
            let item = self.parse_property(bump)?;
            items.push(item);

            match &self.peek_token.kind {
                TokenKind::Comma => self.next_token(),
                TokenKind::RBrace => break,
                _ => {
                    expected_token_err!(
                        self.peek_token,
                        self.lexer.row,
                        self.peek_token.start_column,
                        Comma | RBrace
                    )
                }
            }
        }

        Ok(JsonValue::Object(items))
    }

    fn parse_root_object(&mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let result = self.parse_object(bump)?;

        self.next_token();

        if !matches!(
            (&self.current_token.kind, &self.peek_token.kind),
            (TokenKind::RBrace, TokenKind::Eof)
        ) {
            expected_token_err!(
                self.peek_token,
                self.lexer.row,
                self.peek_token.start_column,
                TokenKind::Eof
            )
        }

        Ok(result)
    }

    fn parse_root_array(&mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        let result = self.parse_array(bump)?;

        self.next_token();

        if !matches!(
            (&self.current_token.kind, &self.peek_token.kind),
            (TokenKind::RBracket, TokenKind::Eof)
        ) {
            expected_token_err!(
                self.peek_token,
                self.lexer.row,
                self.peek_token.start_column,
                TokenKind::Eof
            )
        }

        Ok(result)
    }

    pub fn parse(mut self, bump: &'a Bump) -> Result<JsonValue<'a>, ExpectedTokenError> {
        match self.peek_token.kind {
            TokenKind::LBrace => self.parse_root_object(bump),
            TokenKind::LBracket => self.parse_root_array(bump),
            TokenKind::String
            | TokenKind::Number
            | TokenKind::True
            | TokenKind::False
            | TokenKind::Null => {
                let result = self.parse_value(bump)?;

                self.expect_peek(TokenKind::Eof)?;

                Ok(result)
            }
            _ => expected_token_err!(
                self.peek_token,
                self.lexer.row,
                self.peek_token.start_column,
                String | Number | Null | LBrace | LBracket | True | False
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_top_level_object() {
        let json = r#"
{
	"string": "Hello, world!",
	"number": -42,
	"nested_object": {
		"nested_string": "This is a nested string",
		"nested_number": [100, 3.21865081787e-6, 300, [400, -500, [600, [700, {"secret": 12345}]]]]
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
        let parser = Parser::new(json);

        insta::assert_debug_snapshot!(parser.parse(&bump));
    }

    #[test]
    fn parse_top_level_array() {
        let json = r#"
[
    {
        "one": 1,
        "zero": 0
    }
]
"#;

        let bump = Bump::new();
        let parser = Parser::new(json);

        insta::assert_debug_snapshot!(parser.parse(&bump));
    }

    #[test]
    fn parse_invalid_number() {
        let json = r#"
    {
        "one": 4eee,
        "two": 2
    }
    "#;

        let bump = Bump::new();
        let parser = Parser::new(json);

        insta::assert_debug_snapshot!(parser.parse(&bump));
    }

    #[test]
    fn parse_invalid_utf8() {
        let json = r#"
    {
        "one": 4èee,
        "two": 2
    }
    "#;

        let bump = Bump::new();
        let parser = Parser::new(json);

        insta::assert_debug_snapshot!(parser.parse(&bump));
    }

    #[test]
    fn parse_valid_decimal() {
        let json = r#"
[-65619720000000.61972000000,0.29]
    "#;

        let bump = Bump::new();
        let parser = Parser::new(json);

        insta::assert_debug_snapshot!(parser.parse(&bump));
    }

    #[test]
    fn parse_invalid_octal() {
        let json = r#"
    [-65619720000000.61972000000,029]
        "#;

        let bump = Bump::new();
        let parser = Parser::new(json);

        insta::assert_debug_snapshot!(parser.parse(&bump));
    }

    #[test]
    fn parse_test_data() {
        insta::glob!("../../test_data", "**/*.json", |path| {
            let input =
                fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read {path:?}"));

            let bump = Bump::new();
            let parser = Parser::new(&input);

            insta::assert_debug_snapshot!(parser.parse(&bump));
        });
    }
}
