use crate::token::TokenKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedTokenError {
    pub expected: Vec<TokenKind>,
    pub actual: TokenKind,
    pub origin: String,
    pub invalid_row: usize,
    pub invalid_col: usize,
}

impl std::fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expected = self
            .expected
            .iter()
            .map(|kind| format!("'{}'", kind.to_string().to_uppercase()))
            .collect::<Vec<_>>()
            .join(", ");

        write!(
            f,
            "expected token at row {} column {} to be one of: [{}] but got '{}' instead which is '{}'",
            self.invalid_row, self.invalid_col, expected, self.origin, self.actual
        )
    }
}

impl std::error::Error for ExpectedTokenError {}
