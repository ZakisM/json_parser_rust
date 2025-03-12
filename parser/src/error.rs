use crate::token::TokenKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpectedTokenError {
    pub expected: Vec<TokenKind>,
    pub actual: TokenKind,
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
            .join(" | ");

        // Create a local variable to store the column instead of trying to modify self
        let mut col = self.invalid_col;

        // Update the local column variable if needed based on the token kind
        if let TokenKind::Illegal(Some(IllegalReason::String(illegal_string))) = self.actual {
            match illegal_string {
                IllegalString::UnescapedNewLine(column)
                | IllegalString::UnescapedTab(column)
                | IllegalString::InvalidUnicode(column)
                | IllegalString::InvalidEscape(column)
                | IllegalString::MissingClosingQuote(column) => col = column,
            }
        }

        write!(
            f,
            "expected token at row {} column {} to be one of: ({}) but got '{}' instead",
            self.invalid_row, col, expected, self.actual
        )
    }
}

impl std::error::Error for ExpectedTokenError {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IllegalReason {
    Character(char),
    Number(IllegalNumber),
    String(IllegalString),
}

impl std::fmt::Display for IllegalReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            IllegalReason::Character(c) => &format!("invalid character: '{c}'"),
            IllegalReason::Number(e) => &format!("invalid number: {e}"),
            IllegalReason::String(e) => &format!("invalid string: {e}"),
        };

        write!(f, "{value}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IllegalNumber {
    ParseFloatError,
    LeadingZero,
    MissingExponent,
    MinusMissingDigit,
    MissingFraction,
    InvalidFractionPart,
}

impl std::fmt::Display for IllegalNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            IllegalNumber::ParseFloatError => "parsing number",
            IllegalNumber::LeadingZero => "leading zero",
            IllegalNumber::MissingExponent => "missing exponent",
            IllegalNumber::MinusMissingDigit => "minus must be followed by a digit",
            IllegalNumber::MissingFraction => "missing fraction",
            IllegalNumber::InvalidFractionPart => "invalid fraction part",
        };

        write!(f, "{value}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IllegalString {
    UnescapedNewLine(usize),
    UnescapedTab(usize),
    InvalidUnicode(usize),
    InvalidEscape(usize),
    MissingClosingQuote(usize),
}

impl std::fmt::Display for IllegalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            IllegalString::UnescapedNewLine(_) => "unescaped newline",
            IllegalString::UnescapedTab(_) => "unescaped tab",
            IllegalString::InvalidUnicode(_) => "invalid unicode",
            IllegalString::InvalidEscape(_) => "invalid escape",
            IllegalString::MissingClosingQuote(_) => "missing closing quote",
        };

        write!(f, "{value}")
    }
}

#[macro_export]
macro_rules! illegal_number {
    ($variant:ident) => {
        TokenKind::Illegal(Some(IllegalReason::Number(
            $crate::error::IllegalNumber::$variant,
        )))
    };
}

#[macro_export]
macro_rules! illegal_string {
    ($variant:ident, $read_position:expr) => {
        Some(IllegalReason::String(
            $crate::error::IllegalString::$variant($read_position),
        ))
    };
}
