use crate::{ast::JsonValue, Lexer, Token};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
    peek_token: Token<'a>,
}

macro_rules! expect_token {
    ($self:expr, $variant:ident) => {
        std::mem::discriminant(&$self.peek_token) == std::mem::discriminant(&Token::$variant)
    };
    ($self:expr, $variant:ident()) => {
        std::mem::discriminant(&$self.peek_token)
            == std::mem::discriminant(&Token::$variant(Default::default()))
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

    fn parse_item(&mut self) {
        match self.current_token {
            Token::LBrace => {
                if !expect_token!(self, String()) {
                    return;
                }

                let key = self.current_token;
            }
            _ => todo!(),
        };
    }

    fn parse(mut self) -> JsonValue<'a> {
        let mut items = Vec::new();

        while self.current_token != Token::Eof {
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
	"null": null,
	"array": [1, 2, 3, 4, "five", true],
	"nested_object": {
		"nested_string": "This is a nested string",
		"nested_number": 100,
		"nested_array": [10, 20, 30],
		"nested_boolean": false
	},
	"another_nested_object": { "level1": { "level2": { "key": "value" } } }
}
"#;

        let parser = Parser::new(json.as_bytes());

        dbg!(&parser);
    }
}
