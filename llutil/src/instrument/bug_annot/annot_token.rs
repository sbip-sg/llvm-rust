//! tokens to parse annotations

/// A list of tokens used in the lexer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// An identifier, e.g., x, y, x2
    IDENT(String),

    /// A special character denoting characters we don't need in the annotation
    /// parser
    EscapedChar(String),

    /// A number
    Number(i32),

    /// Token to denote a bug annotation `bug`
    BugId,

    /// Token to denote integer overflow type `integer_overflow`
    IntegerOverflow,

    /// Token to start a comment `/*`
    StartComment,

    /// Token to end a comment `*/`
    EndComment,

    /// Token to denote a whitespace ' '
    Whitespace,

    /// Multiplication operator
    MUL,
}

/// Print `Token` to string for debugging.
pub fn token_to_string(token: &Token) -> String {
    match token {
        Token::MUL => String::from("*"),
        Token::BugId => String::from("BUG"),
        Token::IntegerOverflow => String::from("integer_overflow"),
        Token::StartComment => String::from("/*"),
        Token::EndComment => String::from("*/"),
        Token::IDENT(var) => var.to_string(),
        Token::Number(num) => num.to_string(),
        Token::Whitespace => String::from(" "),
        Token::EscapedChar(string) => string.to_string(),
    }
}

/// Checking whether a token needed in parsing annotations.
pub fn is_important_token(token: &Token) -> bool {
    match token {
        Token::MUL => true,
        Token::BugId => true,
        Token::IntegerOverflow => true,
        Token::StartComment => true,
        Token::EndComment => true,
        Token::IDENT(_) => true,
        Token::Number(_) => true,
        Token::Whitespace => false,
        Token::EscapedChar(_) => false,
    }
}

/// A list of bug types of annotations
/// Current implementation is integer overflow, to add other types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BugType {
    /// bug type of integer overflow
    IntegerOverflow,
    /// not in the list of pre-defined bugs
    Unknown,
}
