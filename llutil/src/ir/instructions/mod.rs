//! Module handling different kinds of instructions.

// Export instruction submodules
mod alloca_inst;
mod binary_operator;
mod branch_inst;
mod call_base;
mod call_inst;
mod callbr_inst;
mod cast_inst;
mod cmp_inst;
mod fcmp_inst;
mod icmp_inst;
mod indirectbr_inst;
mod invoke_inst;
mod load_inst;
mod phi_node;
mod predicate;
mod return_inst;
mod sext_inst;
mod store_inst;
mod switch_inst;
mod terminator_inst;
mod traits;
mod trunc_inst;
mod unary_operator;
mod unreachable_inst;
mod zext_inst;

// Re-export traits and data structures from submodules.
pub use crate::ir::instructions::traits::{
    AnyCall, AnyCast, AnyCmp, AnyCondition, AnyInstruction, AnyTerminator,
    AsInstructionValue,
};
pub use alloca_inst::AllocaInst;
pub use binary_operator::BinaryOperator;
pub use branch_inst::BranchInst;
pub use call_base::CallBase;
pub use call_inst::CallInst;
pub use callbr_inst::CallBrInst;
pub use cast_inst::CastInst;
pub use cmp_inst::CmpInst;
pub use fcmp_inst::FCmpInst;
pub use icmp_inst::ICmpInst;
pub use indirectbr_inst::IndirectBrInst;
pub use invoke_inst::InvokeInst;
pub use load_inst::LoadInst;
pub use phi_node::PhiNode;
pub use predicate::BinaryPredicate::{self, FloatPred, IntPred};
pub use return_inst::ReturnInst;
pub use sext_inst::SExtInst;
pub use store_inst::StoreInst;
pub use switch_inst::SwitchInst;
pub use terminator_inst::TerminatorInst;
pub use trunc_inst::TruncInst;
pub use unary_operator::UnaryOperator;
pub use unreachable_inst::UnreachableInst;
pub use zext_inst::ZExtInst;
