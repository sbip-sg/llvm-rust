//! Module containing extended utilities for LLVM IR.

// Export sub modules
pub mod any_value;
pub mod array_value;
pub mod basic_block;
pub mod basic_value;
pub mod builtin;
pub mod callable_value;
pub mod code_file;
pub mod float;
pub mod function_value;
pub mod instruction;
pub mod instructions;
pub mod int_value;
pub mod metadata_value;
pub mod module;
pub mod path_condition;
pub mod pointer;
pub mod predecessor_block;
pub mod struct_value;
pub mod successor_block;
pub mod vector_value;

// Re-export sub-modules' data structures
pub use crate::ir::instructions::{
    AllocaInst, AnyCall, AnyCast, AnyCmp, AnyCondition, AnyInstruction,
    AnyTerminator, AsInstructionValue, BinaryOperator, BinaryPredicate,
    BranchInst, CallBase, CallBrInst, CallInst, CastInst, CmpInst, FCmpInst,
    FloatPred, ICmpInst, IndirectBrInst, IntPred, InvokeInst, LoadInst,
    PhiNode, ReturnInst, SExtInst, StoreInst, SwitchInst, TerminatorInst,
    TruncInst, UnaryOperator, UnreachableInst, ZExtInst,
};
pub use any_value::AnyValueExt;
pub use array_value::ArrayExt;
pub use basic_block::Blocks;
pub use basic_value::BasicValueExt;
pub use callable_value::CallableExt;
pub use code_file::CodeFile;
pub use float::FloatExt;
pub use function_value::{FunctionExt, FunctionOption, Functions};
pub use instruction::InstructionExt;
pub use int_value::IntExt;
pub use metadata_value::MetadataExt;
pub use module::ModuleExt;
pub use path_condition::PathCondition;
pub use pointer::PointerExt;
pub use predecessor_block::PredecessorBlock;
pub use struct_value::StructExt;
pub use successor_block::SuccessorBlock;
pub use vector_value::VectorExt;
