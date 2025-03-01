use crate::token::TokenKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedTokenError {
    pub expected: Vec<TokenKind>,
    pub actual: TokenKind,
    pub origin: String,
    pub row: usize,
    pub column: usize,
}

impl ExpectedTokenError {
    pub fn with_offset(
        expected: Vec<TokenKind>,
        actual: TokenKind,
        origin: String,
        row: usize,
        column: usize,
    ) -> Self {
        dbg!(&column);
        let column = column - origin.chars().count();

        Self {
            expected,
            actual,
            origin,
            row,
            column,
        }
    }
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
            "expected token at row {} column {} to be {} but got '{}' instead which is '{}'",
            self.row, self.column, expected, self.origin, self.actual
        )
    }
}

impl std::error::Error for ExpectedTokenError {}
