use crate::token::TokenKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedTokenError {
    pub expected: Vec<TokenKind>,
    pub actual: TokenKind,
    pub origin: String,
    pub position: usize,
}

impl std::fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expected = self
            .expected
            .iter()
            .map(|kind| format!("'{kind}'"))
            .collect::<Vec<_>>()
            .join(" OR ");

        write!(
            f,
            "expected token at position {} to be {} but got '{}' instead which is '{}'",
            self.position, expected, self.origin, self.actual
        )
    }
}

impl std::error::Error for ExpectedTokenError {}
