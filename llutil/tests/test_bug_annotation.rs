//! Test bug annotation features

#[cfg(test)]
use llutil::instrument::bug_annot::{
    annot_lexer::{
        escaped_char, identifier, match_decimal, match_special_token,
        match_token_single, Span,
    },
    annot_token,
};

#[test]
fn test_identifier() {
    let input = "xy3 34";
    let result = identifier(Span::new(input)).unwrap();
    let token = annot_token::Token::IDENT("xy3".to_string());
    assert_eq!(result.1.token, token);
    assert_eq!(result.0.to_string(), " 34");
}

#[test]
fn test_escaped_char() {
    let input = "#include";
    let result = escaped_char(Span::new(input)).unwrap();
    let token = annot_token::Token::EscapedChar("#".to_string());
    assert_eq!(result.1.token, token);
    assert_eq!(result.0.to_string(), "include");
}

#[test]
fn test_decimal() {
    let input = "10 + 30";
    let result = match_decimal(Span::new(input)).unwrap();
    let token = annot_token::Token::Number(10);
    assert_eq!(result.1.token, token);
    assert_eq!(result.0.to_string(), " + 30");
}

#[test]
fn test_special_single() {
    let input = "/* bug";
    let result = match_special_token(Span::new(input)).unwrap();
    let token = annot_token::Token::StartComment;
    assert_eq!(result.1.token, token);
    assert_eq!(result.0.to_string(), " bug");
}

#[test]
fn test_token_single() {
    let input = "* 3";
    let result = match_token_single(Span::new(input)).unwrap();
    let token = annot_token::Token::MUL;
    assert_eq!(result.1.token, token);
    assert_eq!(result.0.to_string(), " 3");
}
