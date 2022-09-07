//! Module modelling path condition between two basic blocks.
//!
//! TODO: Move this module to a new super module.

use std::fmt::{Display, Formatter, Result};

use inkwell::values::{BasicValueEnum, AnyValue};

/// Data structure modelling a path condition between two basic blocks.
#[derive(Clone, Debug)]
pub enum PathCondition<'ctx> {
    /// No path condition.
    None,

    /// A Boolean path condition, which consists of a Boolean variable and its
    /// value (`true` or `false`).
    Boolean(BasicValueEnum<'ctx>, bool),

    /// A Value path condition, which consists of a variable and its value.
    Value(BasicValueEnum<'ctx>, BasicValueEnum<'ctx>),
}

/// Implement methods for `PathCondition`.
impl<'ctx> PathCondition<'ctx> {
    /// Constructor
    pub fn empty_condition() -> PathCondition<'ctx> {
        PathCondition::None
    }
}

/// Implement trait `Display` for `PathCondition`.
impl<'ctx> Display for PathCondition<'ctx> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PathCondition::None => write!(f, "None"),
            PathCondition::Boolean(v, b) => {
                if *b {
                    write!(f, "{}", v.print_to_string())
                } else {
                    write!(f, "!{}", v.print_to_string())
                }
            }
            PathCondition::Value(v, u) => write!(f, "{}={}", v, u),
        }
    }
}
