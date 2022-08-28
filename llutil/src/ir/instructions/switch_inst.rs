//! Module handling to the `switch` instruction of LLVM.

use std::convert::TryFrom;
use std::fmt::{self, Display};

use either::Either;
use inkwell::values::{
    AnyValue, AsValueRef, BasicBlock, BasicValueEnum, InstructionValue,
};
use llvm_sys::prelude::LLVMValueRef;

use crate::ir::{PathCondition, SuccessorBlock};

use super::{AnyCondition, AnyInstruction, AnyTerminator, AsInstructionValue};

/// Data structure modelling a `switch` instruction.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct SwitchInst<'ctx> {
    /// Instruction value corresponding to the `SwitchInst`.
    switch_inst: InstructionValue<'ctx>,
}

/// Implement methods for `SwitchInst`.
impl<'ctx> SwitchInst<'ctx> {
    /// Constructor of a `SwitchInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_switch_inst());
        SwitchInst { switch_inst: inst }
    }

    /// Get default successor block.
    pub fn get_default_successor(&self) -> BasicBlock<'ctx> {
        match self.get_successor(0) {
            Some(blk) => blk,
            None => panic!("Invalid switch instruction: {}", self),
        }
    }

    /// Get number of cases, except the default case.
    pub fn get_num_cases(&self) -> u32 {
        (self.get_num_operands() - 2) / 2
    }

    /// Get the value of a switch-case.
    pub fn get_case_value(&self, index: u32) -> BasicValueEnum<'ctx> {
        match self.get_operand(index * 2 + 2) {
            None => panic!(
                "{}{}\n{}{}",
                "Invalid switch instruction: ",
                self,
                "Unable to get value of the case: ",
                index
            ),
            Some(case_value) => match case_value {
                Either::Left(v) => v,
                Either::Right(_) => panic!(
                    "{}{}\n{}{}",
                    "Invalid switch instruction: ",
                    self,
                    "Unable to get value of the case: ",
                    index
                ),
            },
        }
    }

    /// Get the successor of a switch-case.
    pub fn get_case_successor(&self, index: u32) -> BasicBlock<'ctx> {
        match self.get_operand(index * 2 + 3) {
            None => panic!(
                "{}{}\n{}{}",
                "Invalid switch instruction: ",
                self,
                "Unable to get value of the case: ",
                index
            ),
            Some(case_value) => match case_value {
                Either::Left(_) => panic!(
                    "{}{}\n{}{}",
                    "Invalid switch instruction: ",
                    self,
                    "Unable to get value of the case: ",
                    index
                ),
                Either::Right(blk) => blk,
            },
        }
    }

    /// Get all successor blocks with path conditions.
    pub fn get_conditioned_successors(&self) -> Vec<SuccessorBlock<'ctx>> {
        let mut successors = vec![];

        let default_sblk = SuccessorBlock::new(
            PathCondition::None,
            self.get_default_successor(),
        );
        successors.push(default_sblk);

        let cond_value = self.get_condition();
        for i in 0..self.get_num_cases() {
            let case_value = self.get_case_value(i);
            let case_blk = self.get_case_successor(i);
            let case_sblk = SuccessorBlock::new(
                PathCondition::Value(cond_value, case_value),
                case_blk,
            );
            successors.push(case_sblk)
        }

        successors
    }
}

/// Implement the `AsInstructionValue` trait for `SwitchInst`.
impl<'ctx> AsInstructionValue<'ctx> for SwitchInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.switch_inst
    }
}

/// Implement the `AsValueRef` trait for `SwitchInst`.
impl<'ctx> AsValueRef for SwitchInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.switch_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `SwitchInst`.
impl<'ctx> AnyInstruction<'ctx> for SwitchInst<'ctx> {}

/// Implement the `AnyTerminator` trait for `SwitchInst`.
impl<'ctx> AnyTerminator<'ctx> for SwitchInst<'ctx> {}

/// Implement the `AnyCondition` trait for `SwitchInst`.
impl<'ctx> AnyCondition<'ctx> for SwitchInst<'ctx> {}

/// Implement the `AnyValue` trait for `SwitchInst`.
impl<'ctx> AnyValue<'ctx> for SwitchInst<'ctx> {}

/// Implement the `Display` trait for `SwitchInst`.
impl<'ctx> Display for SwitchInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `SwitchInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for SwitchInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_switch_inst() {
            Ok(SwitchInst::new(inst))
        } else {
            Err(())
        }
    }
}
