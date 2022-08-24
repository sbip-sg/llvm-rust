use llvm_sys::prelude::LLVMValueRef;

use std::ffi::CStr;
use std::fmt::{self, Display};

use crate::types::StructType;
use crate::values::traits::AsValueRef;
use crate::values::{InstructionValue, Value};

use super::AnyValue;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StructValue<'ctx> {
    struct_value: Value<'ctx>,
}

impl<'ctx> StructValue<'ctx> {
    pub(crate) unsafe fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        StructValue {
            struct_value: Value::new(value),
        }
    }

    /// Gets the name of a `StructValue`. If the value is a constant, this will
    /// return an empty string.
    pub fn get_name(&self) -> &CStr {
        self.struct_value.get_name()
    }

    /// Get name of the `StructValue` or return a default name.
    pub fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-struct-name>".to_string(),
        }
    }

    /// Get name of the `StructValue`.
    pub fn set_name(&self, name: &str) {
        self.struct_value.set_name(name)
    }

    pub fn get_type(self) -> StructType<'ctx> {
        unsafe { StructType::new(self.struct_value.get_type()) }
    }

    pub fn is_null(self) -> bool {
        self.struct_value.is_null()
    }

    pub fn is_undef(self) -> bool {
        self.struct_value.is_undef()
    }

    pub fn print_to_stderr(self) {
        self.struct_value.print_to_stderr()
    }

    pub fn is_instruction(self) -> bool {
        self.struct_value.is_instruction()
    }

    pub fn as_instruction(self) -> Option<InstructionValue<'ctx>> {
        self.struct_value.as_instruction()
    }

    pub fn replace_all_uses_with(self, other: StructValue<'ctx>) {
        self.struct_value
            .replace_all_uses_with(other.as_value_ref())
    }
}

impl<'ctx> AsValueRef for StructValue<'ctx> {
    fn as_value_ref(&self) -> LLVMValueRef {
        self.struct_value.value
    }
}

impl<'ctx> Display for StructValue<'ctx> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.print_to_llvm_string())
    }
}
