//! Module defining traits handling instructions.

use super::BinaryPredicate;
use crate::{
    types::{AnyTypeEnum, BasicTypeEnum},
    values::{
        AsValueRef, BasicBlock, BasicValueEnum, FunctionValue,
        InstructionOpcode, InstructionValue, PointerValue,
    },
};
use either::Either;
use llvm_sys::core::{
    LLVMGetCondition, LLVMGetNumSuccessors, LLVMGetSuccessor, LLVMIsConditional,
};
use std::{
    ffi::{CStr, CString},
    fmt::Display,
    panic,
};
use Either::Left;

/// Trait to convert a specific instruction into an `InstructionValue`.
pub trait AsInstructionValue<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx>;
}

/// Trait provides common utility functions of all instructions.
pub trait AnyInstruction<'ctx>: AsInstructionValue<'ctx> {
    /// Get type of the instruction.
    fn get_type(&self) -> AnyTypeEnum<'ctx> {
        self.as_instruction_value().get_type()
    }

    /// Get opcode of the instruction.
    fn get_opcode(&self) -> InstructionOpcode {
        self.as_instruction_value().get_opcode()
    }

    /// Get number of operands.
    fn get_num_operands(&self) -> u32 {
        self.as_instruction_value().get_num_operands()
    }

    /// Get an operand.
    fn get_operand(
        &self,
        index: u32,
    ) -> Option<Either<BasicValueEnum<'ctx>, BasicBlock<'ctx>>> {
        self.as_instruction_value().get_operand(index)
    }

    // /// Check if the current instruction returns a signed integer.
    // fn is_signed_integer(&self) -> bool {
    //     match self.get_opcode() {
    //         InstructionOpcode::Sub

    //     }
    // }
}

/// Trait providing utility functions to handle function call instructions,
/// including `CallInst`, `CallBrInst`, `Invoke`, and `CallBase`.
pub trait AnyCall<'ctx>: AnyInstruction<'ctx> + Sized + Display {
    /// Get the called operand of a function call instruction.
    ///
    /// The returned value is a `PointerValue` pointing to either a function
    /// definition or a function pointer.
    fn get_called_operand(&self) -> PointerValue<'ctx> {
        let num_operands = self.get_num_operands();
        let callee = self.get_operand(num_operands - 1);

        if let Some(Left(callee)) = callee {
            if callee.is_pointer_value() {
                callee.into_pointer_value()
            } else {
                panic!("Invalid function call instruction: {}", self);
            }
        } else {
            panic!("Invalid function call instruction: {}", self);
        }
    }

    /// Get the arguments a function call instruction.
    ///
    /// The returned value is vector of called arguments.
    fn get_called_arguments(&self) -> Vec<BasicValueEnum<'ctx>> {
        match self.get_opcode() {
            InstructionOpcode::Call => {
                let mut res = vec![];
                let n = self.get_num_operands();
                for i in 0..(n - 1) {
                    res.push(self.get_operand(i).unwrap().left().unwrap());
                }
                res
            }

            InstructionOpcode::CallBr => {
                todo!("get_callee_arguments: handle CallBr");
                vec![]
            }

            InstructionOpcode::Invoke => {
                todo!("get_callee_arguments: handle CallBr");
                vec![]
            }

            _ => vec![],
        }
    }

    /// Get name of the called operand.
    fn get_called_operand_name(&self) -> Option<String> {
        match self.get_called_operand().get_name().to_str() {
            Ok(name) => Some(name.to_owned()),
            Err(_) => None,
        }
    }

    /// Get the called function of a function call instruction.
    ///
    /// The returned value is None if this is an indirect function call (the
    /// called operand is a function pointer).
    fn get_called_function(&self) -> Option<FunctionValue<'ctx>> {
        let callee = self.get_called_operand();
        callee.as_function()
    }
}

/// Trait providing utility functions to handle comparison instructions such as
/// `CmpInst`, `FCmpInst` and `ICmpInst`.
pub trait AnyCmp<'ctx>: AnyInstruction<'ctx> + Display + Sized {
    /// Get comparison predicate
    fn get_predicate(&'ctx self) -> BinaryPredicate {
        if self.get_type().is_int_type() {
            if let Some(p) = self.as_instruction_value().get_icmp_predicate() {
                return BinaryPredicate::IntPred(p);
            }
        } else if self.get_type().is_float_type() {
            if let Some(p) = self.as_instruction_value().get_fcmp_predicate() {
                return BinaryPredicate::FloatPred(p);
            }
        }
        panic!(
            "Invalid comparison instruction: {}\n{}",
            self, "Unable to get the comparison predicate!"
        )
    }

    /// Get the first operand of the comparison instruction.
    fn get_first_operand(self) -> BasicValueEnum<'ctx> {
        if let Some(opr) = self.get_operand(0) {
            if let Left(v) = opr {
                return v;
            }
        }

        panic!(
            "Invalid comparison instruction: {}\n{}",
            self, "Unable to get the first operand!"
        )
    }

    /// Get the second operand of the comparison instruction.
    fn get_second_operand(self) -> BasicValueEnum<'ctx> {
        if let Some(opr) = self.get_operand(1) {
            if let Left(v) = opr {
                return v;
            }
        }

        panic!(
            "Invalid comparison instruction: {}\n{}",
            self, "Unable to get the second operand!"
        )
    }
}

/// Trait providing utility functions to handle casting instructions, including
/// integer casting, pointer casting.
pub trait AnyCast<'ctx>: AnyInstruction<'ctx> + Display + Sized {
    /// Get opcode of the cast instruction.
    fn get_opcode(&self) -> InstructionOpcode {
        self.as_instruction_value().get_opcode()
    }

    /// Get the source operand of the cast instruction.
    fn get_source_operand(self) -> BasicValueEnum<'ctx> {
        if let Some(opr) = self.get_operand(0) {
            if let Left(v) = opr {
                return v;
            }
        }

        panic!(
            "Invalid casting instruction: {}\n{}",
            self, "Unable to get the source operand!"
        )
    }

    /// Get the source type of the cast instruction.
    fn get_source_type(self) -> BasicTypeEnum<'ctx> {
        if let Some(opr) = self.get_operand(0) {
            if let Left(v) = opr {
                return v.get_type();
            }
        }

        panic!(
            "Invalid casting instruction: {}\n{}",
            self, "Unable to get the source type!"
        )
    }

    /// Get the destination type of the cast instruction.
    fn get_destination_type(self) -> BasicTypeEnum<'ctx>
    where
        Self: std::panic::RefUnwindSafe,
    {
        let res = panic::catch_unwind(|| self.get_type().to_basic_type_enum());
        match res {
            Ok(typ) => typ,
            Err(_) => panic!(
                "Invalid casting instruction: {}\n{}",
                self, "Unable to get the destination type!"
            ),
        }
    }
}

/// Trait providing utility functions to handle terminator instructions.
pub trait AnyTerminator<'ctx>: AsValueRef {
    /// Get the number of successor blocks of the current `TerminatorInst`.
    fn get_num_successors(&self) -> u32 {
        unsafe { LLVMGetNumSuccessors(self.as_value_ref()) }
    }

    /// Get a successor block of the current `TerminatorInst`.
    fn get_successor(&self, index: u32) -> Option<BasicBlock<'ctx>> {
        unsafe {
            let successor = LLVMGetSuccessor(self.as_value_ref(), index);
            BasicBlock::new(successor)
        }
    }

    /// Get all successor blocks of the current `TerminatorInst`.
    fn get_successors(&self) -> Vec<BasicBlock<'ctx>> {
        let mut successors = vec![];

        let num_succs = self.get_num_successors();
        if num_succs > 0 {
            for i in 0..num_succs {
                if let Some(blk) = self.get_successor(i) {
                    successors.push(blk)
                }
            }
        }

        successors
    }
}

/// Trait providing utility functions to handle conditional and
/// unconditional instructions.
pub trait AnyCondition<'ctx>:
    AsValueRef + Display + AnyTerminator<'ctx>
{
    /// Check if the current `TerminatorInst` has a conditional expression.
    fn has_condition(&self) -> bool {
        unsafe { LLVMIsConditional(self.as_value_ref()) != 0 }
    }

    /// Get conditional expression of the current `TerminatorInst`.
    ///
    /// Applicable if the current `TerminatorInst` is a `BranchInst`.
    fn get_condition(&self) -> BasicValueEnum<'ctx> {
        unsafe {
            if self.has_condition() {
                let condition = LLVMGetCondition(self.as_value_ref());
                BasicValueEnum::new(condition)
            } else {
                panic!("Expect conditional instruction: {}", self)
            }
        }
    }
}
