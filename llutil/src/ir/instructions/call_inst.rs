//! Module handling to the `call` instruction of LLVM.

use std::fmt::{self, Display};

use inkwell::values::{AnyValue, AsValueRef, FunctionValue, InstructionValue};
use llvm_sys::prelude::LLVMValueRef;

use super::{AnyCall, AnyInstruction, AsInstructionValue};

/// Data structure modelling a `call` instruction.
///
/// LLVM Language Reference Manual for the [`call`
/// instruction](https://llvm.org/docs/LangRef.html#call-instruction)
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct CallInst<'ctx> {
    /// Instruction value corresponding to the `CallInst`.
    call_inst: InstructionValue<'ctx>,
}

/// Implement methods for `CallInst`
impl<'ctx> CallInst<'ctx> {
    /// Constructor of a `CallInst` instruction.
    pub fn new(inst: InstructionValue<'ctx>) -> Self {
        debug_assert!(inst.is_a_call_inst());
        CallInst { call_inst: inst }
    }

    /// Find the called function.
    pub fn get_called_fn_value(self) -> FunctionValue<'ctx> {
        use llvm_sys::core::LLVMGetCalledValue;

        unsafe {
            FunctionValue::new(LLVMGetCalledValue(self.as_value_ref()))
                .expect("This shoud nevel be null?")
        }
    }
}

/// Implement the `AsInstructionValue` trait for `CallInst`.
impl<'ctx> AsInstructionValue<'ctx> for CallInst<'ctx> {
    fn as_instruction_value(&self) -> InstructionValue<'ctx> {
        self.call_inst
    }
}

/// Implement the `AsValueRef` trait for `CallInst`.
impl<'ctx> AsValueRef for CallInst<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.call_inst.as_value_ref()
    }
}

/// Implement the `AnyInstruction` trait for `CallInst`.
impl<'ctx> AnyInstruction<'ctx> for CallInst<'ctx> {}

/// Implement the `AnyCall` trait for `CallInst`.
impl<'ctx> AnyCall<'ctx> for CallInst<'ctx> {}

/// Implement the `AnyValue` trait for `CallInst`.
impl<'ctx> AnyValue<'ctx> for CallInst<'ctx> {}

/// Implement the `Display` trait for `CallInst`.
impl<'ctx> Display for CallInst<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}

/// Implement the `TryFrom` trait for `CallInst`.
impl<'ctx> TryFrom<InstructionValue<'ctx>> for CallInst<'ctx> {
    type Error = ();

    fn try_from(inst: InstructionValue<'ctx>) -> Result<Self, Self::Error> {
        if inst.is_a_call_inst() {
            Ok(CallInst::new(inst))
        } else {
            Err(())
        }
    }
}
