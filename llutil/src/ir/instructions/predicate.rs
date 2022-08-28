//! Module handling predicates of comparison instructions.

use std::fmt::{Display, Formatter, Result};

use inkwell::{FloatPredicate, IntPredicate};

/// Data structure modelling binary predicates between two expressions.
#[derive(Clone, Copy, Debug)]
pub enum BinaryPredicate {
    /// Predicates for comparing integer and pointer expressions.
    IntPred(IntPredicate),

    /// Predicates over float expressions.
    FloatPred(FloatPredicate),
}

/// Utility functions for handling `BinaryPredicate`.
impl BinaryPredicate {
    /// Negate a predicate.
    pub fn negate(&self) -> Self {
        match self {
            BinaryPredicate::IntPred(pred) => {
                let neg_pred = match pred {
                    IntPredicate::EQ => IntPredicate::NE,
                    IntPredicate::NE => IntPredicate::EQ,
                    IntPredicate::UGT => IntPredicate::ULE,
                    IntPredicate::UGE => IntPredicate::ULT,
                    IntPredicate::ULT => IntPredicate::UGE,
                    IntPredicate::ULE => IntPredicate::UGT,
                    IntPredicate::SGT => IntPredicate::SLE,
                    IntPredicate::SGE => IntPredicate::SLT,
                    IntPredicate::SLT => IntPredicate::SGE,
                    IntPredicate::SLE => IntPredicate::SGT,
                };
                BinaryPredicate::IntPred(neg_pred)
            }
            BinaryPredicate::FloatPred(pred) => {
                let neg_pred = match pred {
                    FloatPredicate::OEQ => FloatPredicate::UNE,
                    FloatPredicate::OGE => FloatPredicate::ULT,
                    FloatPredicate::OGT => FloatPredicate::ULE,
                    FloatPredicate::OLE => FloatPredicate::UGT,
                    FloatPredicate::OLT => FloatPredicate::UGE,
                    FloatPredicate::ONE => FloatPredicate::UEQ,
                    FloatPredicate::ORD => FloatPredicate::UNO,
                    FloatPredicate::PredicateFalse => {
                        FloatPredicate::PredicateTrue
                    }
                    FloatPredicate::PredicateTrue => {
                        FloatPredicate::PredicateFalse
                    }
                    FloatPredicate::UEQ => FloatPredicate::ONE,
                    FloatPredicate::UGE => FloatPredicate::OLT,
                    FloatPredicate::UGT => FloatPredicate::OLE,
                    FloatPredicate::ULE => FloatPredicate::OGT,
                    FloatPredicate::ULT => FloatPredicate::OGE,
                    FloatPredicate::UNE => FloatPredicate::OEQ,
                    FloatPredicate::UNO => FloatPredicate::ORD,
                };
                BinaryPredicate::FloatPred(neg_pred)
            }
        }
    }
}

/// Implement the `Display` trait for `BinaryPredicate`.
impl Display for BinaryPredicate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BinaryPredicate::IntPred(pred) => match pred {
                IntPredicate::EQ => write!(f, "="),
                IntPredicate::NE => write!(f, "!="),
                IntPredicate::UGT | IntPredicate::SGT => write!(f, ">"),
                IntPredicate::UGE | IntPredicate::SGE => write!(f, ">="),
                IntPredicate::ULT | IntPredicate::SLT => write!(f, "<"),
                IntPredicate::ULE | IntPredicate::SLE => write!(f, "<="),
            },
            BinaryPredicate::FloatPred(pred) => match pred {
                FloatPredicate::OEQ | FloatPredicate::UEQ => write!(f, "="),
                FloatPredicate::OGE | FloatPredicate::UGE => write!(f, ">="),
                FloatPredicate::OGT | FloatPredicate::UGT => write!(f, ">"),
                FloatPredicate::OLE | FloatPredicate::ULE => write!(f, "<="),
                FloatPredicate::OLT | FloatPredicate::ULT => write!(f, "<"),
                FloatPredicate::ONE | FloatPredicate::UNE => write!(f, "!="),
                FloatPredicate::ORD => write!(f, "!=NaN"),
                FloatPredicate::PredicateFalse => write!(f, "false"),
                FloatPredicate::PredicateTrue => write!(f, "true"),
                FloatPredicate::UNO => write!(f, "=NaN"),
            },
        }
    }
}
