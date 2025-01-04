use crate::TokenKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ParserErrors(Vec<ExpectedTokenError>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ParserErrors(errors) => {
                let errors = errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("\n\n");

                writeln!(f, "parsing failed:")?;
                write!(f, "{errors}")
            }
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedTokenError {
    pub expected: Vec<TokenKind>,
    pub actual: TokenKind,
}

impl std::fmt::Display for ExpectedTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expected = self
            .expected
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" | ");

        write!(
            f,
            "expected next token to be {}, got {} instead",
            expected, self.actual
        )
    }
}

impl std::error::Error for ExpectedTokenError {}
