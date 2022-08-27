//! Module handling to the `phi` instruction of LLVM.

use crate::values::{
    AnyValue, AsValueRef, BasicBlock, BasicValue, BasicValueEnum,
    FunctionValue, InstructionValue, PointerValue,
};
use either::Either::{Left, Right};
use llvm_sys::{
    core::{
        LLVMAddIncoming, LLVMCountIncoming, LLVMGetIncomingBlock,
        LLVMGetIncomingValue,
    },
    prelude::{LLVMBasicBlockRef, LLVMValueRef},
};
use std::ffi::CStr;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

use super::{AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling a `phi` instruction.
///
/// REVIEW: merge this module with PhiValue?
#[derive(Debug, PartialEq, Eq, Copy, Hash)]
pub struct PhiNode<'ctx> {
    phi_node: InstructionValue<'ctx>,
}

/// Implement methods for `PhiNode`.
impl<'ctx> PhiNode<'ctx> {
    /// Constructor of a `PhiNode` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_phi_node());
        PhiNode { phi_node: inst }
    }

    /// Get name of the instruction.
    pub fn get_name(&self) -> Option<&CStr> {
        self.phi_node.get_name()
    }

    /// Set name of the instruction.
    pub fn set_name(&self, name: &str) -> Result<(), &'static str> {
        self.as_instruction_value().set_name(name)
    }

    /// Count the number of incoming value and basic block pairs.
    pub fn count_incoming(self) -> u32 {
        unsafe { LLVMCountIncoming(self.as_value_ref()) }
    }

    /// Get a pair of incoming value and basic block.x
    pub fn get_incoming(
        self,
        index: u32,
    ) -> Option<(BasicValueEnum<'ctx>, BasicBlock<'ctx>)> {
        if index >= self.count_incoming() {
            return None;
        }

        let basic_block = unsafe {
            BasicBlock::new(LLVMGetIncomingBlock(self.as_value_ref(), index))
                .expect("Invalid BasicBlock")
        };
        let value = unsafe {
            BasicValueEnum::new(LLVMGetIncomingValue(
                self.as_value_ref(),
                index,
            ))
        };

        Some((value, basic_block))
    }

    /// Add a pair of incoming value and basic block.
    pub fn add_incoming(
        self,
        incoming: &[(&dyn BasicValue<'ctx>, BasicBlock<'ctx>)],
    ) {
        let (mut values, mut basic_blocks): (
            Vec<LLVMValueRef>,
            Vec<LLVMBasicBlockRef>,
        ) = {
            incoming
                .iter()
                .map(|&(v, bb)| (v.as_value_ref(), bb.basic_block))
                .unzip()
        };

        unsafe {
            LLVMAddIncoming(
                self.as_value_ref(),
                values.as_mut_ptr(),
                basic_blocks.as_mut_ptr(),
                incoming.len() as u32,
            );
        }
    }

    /// Get all pairs of incoming values and basic blocks.
    pub fn get_incomings(
        self,
    ) -> Vec<(BasicValueEnum<'ctx>, BasicBlock<'ctx>)> {
        let mut incomings = vec![];

        for i in 0..self.count_incoming() {
            let basic_block = unsafe {
                BasicBlock::new(LLVMGetIncomingBlock(self.as_value_ref(), i))
                    .expect("Invalid BasicBlock")
            };
            let value = unsafe {
                BasicValueEnum::new(LLVMGetIncomingValue(
                    self.as_value_ref(),
                    i,
                ))
            };

            incomings.push((value, basic_block))
        }

        incomings
    }

    /// Replace all uses of the `PhiNode`.
    pub fn replace_all_uses_with(self, other: &PhiNode<'ctx>) {
        self.phi_node
            .replace_all_uses_with(&other.as_instruction_value())
    }
}

/// Module containing all implementations of the conversion traits.
pub mod conversion_traits {
    use super::PhiNode;
    use crate::values::{
        instructions::AsInstructionValue, AsValueRef, InstructionValue,
    };
    use llvm_sys::prelude::LLVMValueRef;

    /// Implement the `AsInstructionValue` trait for `PhiNode`.
    impl<'ctx> AsInstructionValue<'ctx> for PhiNode<'ctx> {
        fn as_instruction_value(&self) -> InstructionValue<'ctx> {
            self.phi_node
        }
    }

    /// Implement the `AsValueRef` trait for `PhiNode`.
    impl<'ctx> AsValueRef for PhiNode<'ctx> {
        fn as_value_ref(&self) -> LLVMValueRef {
            self.phi_node.as_value_ref()
        }
    }
}

/// Module containing all implementations of the behaviour traits.
pub mod behaviour_traits {
    use super::PhiNode;
    use crate::values::instructions::AnyInstruction;

    /// Implement the `AnyInstruction` trait for `PhiNode`.
    impl<'ctx> AnyInstruction<'ctx> for PhiNode<'ctx> {}
}

/// Module containing all implementations of the standard traits.
pub mod standard_traits {
    use super::PhiNode;
    use crate::values::{AnyValue, InstructionValue};
    use std::{
        convert::TryFrom,
        fmt::{self, Display},
    };

    /// Implement the `Display` trait for `PhiNode`.
    impl<'ctx> Display for PhiNode<'ctx> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.print_to_llvm_string())
        }
    }

    /// Implement the `Clone` trait for `PhiNode`.
    impl<'ctx> Clone for PhiNode<'ctx> {
        fn clone(&self) -> Self {
            Self {
                phi_node: self.phi_node.clone(),
            }
        }
    }

    /// Implement the `TryFrom` trait for `PhiNode`.
    impl<'ctx> TryFrom<InstructionValue<'ctx>> for PhiNode<'ctx> {
        type Error = ();

        fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
            if inst.is_a_phi_node() {
                unsafe { Ok(PhiNode::new(inst)) }
            } else {
                Err(())
            }
        }
    }
}
