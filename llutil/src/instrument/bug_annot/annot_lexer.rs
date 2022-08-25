//! Module implementing lexer for bug annotations

use crate::instrument::bug_annot::annot_token;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, one_of},
    combinator::{recognize, value},
    multi::{many0, many1},
    sequence::{pair, terminated},
};

use nom_locate::{position, LocatedSpan};

/// Using `nom_locate` for location information
pub type Span<'a> = LocatedSpan<&'a str>;

/// Data structure for generating error information when parsing fails.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError<'a> {
    /// Error location.
    span: Span<'a>,

    /// Error message.
    message: Option<String>,
}

/// Change the default `IResult` of nom to capture the parse error information.
pub type IResult<'a, O> = nom::IResult<Span<'a>, O, ParseError<'a>>;

impl<'a> ParseError<'a> {
    /// new error when cannot parse
    pub fn new(message: String, span: Span<'a>) -> Self {
        Self {
            span,
            message: Some(message),
        }
    }

    /// the error
    #[inline]
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// the line number of the input string when cannot parse
    #[inline]
    pub fn line(&self) -> u32 {
        self.span().location_line()
    }

    /// the location offset of the input string when cannot parse
    pub fn offset(&self) -> usize {
        self.span().location_offset()
    }
}

/// This part implement functions of `nom::error::ParseError` since we customize
/// parse.
impl<'a> nom::error::ParseError<Span<'a>> for ParseError<'a> {
    fn from_error_kind(input: Span<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::new(format!("parse error {:?}", kind), input)
    }

    fn append(
        _input: Span<'a>,
        _kind: nom::error::ErrorKind,
        other: Self,
    ) -> Self {
        other
    }

    fn from_char(input: Span<'a>, c: char) -> Self {
        Self::new(format!("unexpected character '{}'", c), input)
    }
}

/// Data structure containing token and its location information.
#[derive(Clone)]
pub struct TokenInfo<'a> {
    /// An annotation token
    pub token: annot_token::Token,
    /// the position in the input of the token
    pub position: Span<'a>,
}

/// A customized printing function to be more concise
impl std::fmt::Debug for TokenInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenInfo")
            .field("token", &self.token)
            .field("line_number", &self.position.location_line())
            .field("col_number", &self.position.get_column())
            .finish()
    }
}

/// The lexer to do lexing for the annotations
pub fn identifier(input: Span) -> IResult<TokenInfo> {
    // [a-zA-Z_][a-zA-Z0-9_]*
    let (rest, m) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;

    let (_, pos) = position::<Span, ParseError>(input).unwrap();
    let token = TokenInfo {
        token: annot_token::Token::IDENT(m.to_string()),
        position: pos,
    };
    Ok((rest, token))
}

/// matching characters we don't need to find the annotations
pub fn escaped_char(input: Span) -> IResult<TokenInfo> {
    let escaped = "#<>%.(){},*=;\"&:/@[]-^+`\\!";
    let result =
        recognize(many1(terminated(one_of(escaped), many0(char('_')))))(input);

    match result {
        Ok((rest, string)) => {
            let (_, pos) = position::<Span, ParseError>(input).unwrap();
            let token = TokenInfo {
                token: annot_token::Token::EscapedChar(string.to_string()),
                position: pos,
            };
            Ok((rest, token))
        }
        Err(msg) => Err(msg),
    }
}

/// matching decimal numbers
pub fn match_decimal(input: Span) -> IResult<TokenInfo> {
    let result = recognize(many1(terminated(
        one_of("0123456789"),
        many0(char('_')),
    )))(input);

    match result {
        Ok((rest, num)) => {
            let (_, pos) = position::<Span, ParseError>(input).unwrap();
            let token = TokenInfo {
                token: annot_token::Token::Number(num.parse::<i32>().unwrap()),
                position: pos,
            };
            Ok((rest, token))
        }
        Err(msg) => Err(msg),
    }
}

/// matching tokens of single characters
pub fn match_token_single(input: Span) -> IResult<TokenInfo> {
    let result: IResult<annot_token::Token> = alt((
        value(annot_token::Token::MUL, tag("*")),
        value(annot_token::Token::Whitespace, tag(" ")),
        value(annot_token::Token::Whitespace, tag("\t")),
        value(annot_token::Token::Whitespace, tag("\n")),
    ))(input);

    if let Ok((rest, annot_token)) = result {
        let (_, pos) = position::<Span, ParseError>(input).unwrap();
        let token = TokenInfo {
            token: annot_token,
            position: pos,
        };
        return Ok((rest, token));
    }
    escaped_char(input)
}

/// matching special tokens with high priority
pub fn match_special_token(input: Span) -> IResult<TokenInfo> {
    let result = alt((
        value(annot_token::Token::StartComment, tag("/*")),
        value(annot_token::Token::EndComment, tag("*/")),
        value(annot_token::Token::BugId, tag("bug")),
        value(annot_token::Token::IntegerOverflow, tag("integer_overflow")),
    ))(input);
    match result {
        Ok((rest, annot_token)) => {
            let (_, pos) = position::<Span, ParseError>(input).unwrap();
            let token = TokenInfo {
                token: annot_token,
                position: pos,
            };
            Ok((rest, token))
        }
        Err(msg) => Err(msg),
    }
}

/// the overall matching algorithm to produce tokens
pub fn match_token(input: Span) -> IResult<TokenInfo> {
    let mut result = match_special_token(input);
    if result.is_err() {
        result = match_token_single(input);
        if result.is_err() {
            result = match_decimal(input);
            if result.is_err() {
                return identifier(input);
            }
        }
    }
    result
}

/// The main function of the lexer to get tokens from the input
pub fn nom_lexing(input: Span) -> Vec<TokenInfo> {
    let result = match_token(input);
    if let Ok((n_input, token)) = result {
        // base case: when the input is empty
        if n_input.to_string().is_empty() {
            vec![token]
        } else {
            // continue to process the rest of the input string
            let mut tokens = nom_lexing(n_input);
            tokens.push(token);
            tokens
        }
    } else {
        // exit when cannot parse the input
        panic!("failing to parse the token: {}", input)
    }
}

/// Filter all non-important tokens.
pub fn nom_lexing_filtered(input: &str) -> Vec<TokenInfo> {
    let tokens = nom_lexing(Span::new(input));
    let mut filtered_tokens = Vec::new();
    for token in tokens {
        if annot_token::is_important_token(&token.token) {
            filtered_tokens.push(token);
        }
    }
    filtered_tokens.reverse();
    filtered_tokens
}
