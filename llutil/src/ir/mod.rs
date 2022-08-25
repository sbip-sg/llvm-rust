//! Module containing extended utilities for LLVM IR.

// Export sub modules
pub mod basic_block;
pub mod builtin;
pub mod function;

// Re-export sub-modules' data structures
pub use basic_block::Blocks;
pub use function::{FunctionExt, FunctionOption, FunctionVec};
