//! Parser for bug annotation.

use crate::instrument::bug_annot::annot_lexer;
use crate::instrument::bug_annot::annot_token;

/// Data structure containing information a bug annotation.
#[derive(Debug, Clone)]
pub struct Annotation<'a> {
    /// The annotated bug type.
    pub bug_type: annot_token::BugType,

    /// The list of identifiers, with line number of column number.
    tokens: Vec<annot_lexer::TokenInfo<'a>>,
}

impl<'a> Annotation<'a> {
    /// Create a new bug annotation.
    pub fn new(b_type: annot_token::BugType) -> Self {
        Self {
            bug_type: b_type,
            tokens: Vec::new(),
        }
    }

    /// Update the type of annotation.
    pub fn update_type(&mut self, b_type: annot_token::BugType) {
        self.bug_type = b_type
    }

    /// Adding identifiers to the annotation.
    pub fn update_identifiers(
        &mut self,
        tokens: Vec<annot_lexer::TokenInfo<'a>>,
    ) {
        self.tokens = tokens;
    }
}

/// Post-process the annotations
pub fn parse_annotation(
    annot: Annotation,
) -> Option<(annot_token::BugType, u32, u32)> {
    if annot.bug_type == annot_token::BugType::IntegerOverflow
        && annot.tokens.len() == 3
    {
        let line = annot.tokens[1].position.location_line();
        let col = annot.tokens[1].position.get_column();
        Some((annot_token::BugType::IntegerOverflow, line, (col as u32)))
    } else {
        None
    }
}

/// Parsing tokens from the lexer to get bug annotations
pub fn parsing(
    tokens: Vec<annot_lexer::TokenInfo>,
) -> Vec<(annot_token::BugType, u32, u32)> {
    let mut annotations: Vec<Annotation> = Vec::new();
    let mut comment_start = false;
    let mut bug_annotation_start = false;
    let mut elements = Vec::new();
    let mut annotation = Annotation::new(annot_token::BugType::Unknown);
    for token in tokens {
        match token.token {
            annot_token::Token::StartComment => {
                if bug_annotation_start {
                    annotation.update_identifiers(elements);
                    elements = Vec::new();
                    annotations.push(annotation);
                    annotation = Annotation::new(annot_token::BugType::Unknown);
                    bug_annotation_start = false;
                }
                comment_start = true;
            }
            annot_token::Token::EndComment => {
                comment_start = false;
            }
            annot_token::Token::IntegerOverflow => {
                if comment_start {
                    let bug_type = annot_token::BugType::IntegerOverflow;
                    annotation.update_type(bug_type);
                    bug_annotation_start = true;
                }
            }
            _ => {
                if bug_annotation_start {
                    elements.push(token);
                }
            }
        }
    }

    let mut annot_pairs = Vec::new();
    for annot in annotations {
        if let Some(triple) = parse_annotation(annot) {
            annot_pairs.push(triple)
        }
    }

    annot_pairs
}
