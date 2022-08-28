//! Module provide additional utilities to handle LLVM `AnyValueEnum`.

use inkwell::values::AnyValueEnum;

use super::{
    ArrayExt, FloatExt, FunctionExt, InstructionExt, IntExt, MetadataExt,
    PointerExt, StructExt, VectorExt,
};

/// Trait providing additional functions to handle `AnyValueEnum`.
pub trait AnyValueExt {
    /// Get name of the `AnyValueEnum` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `AnyValueExt` for `AnyValueEnum`.
impl<'ctx> AnyValueExt for AnyValueEnum<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self {
            AnyValueEnum::ArrayValue(v) => v.get_name_or_default(),
            AnyValueEnum::IntValue(v) => v.get_name_or_default(),
            AnyValueEnum::FloatValue(v) => v.get_name_or_default(),
            AnyValueEnum::PointerValue(v) => v.get_name_or_default(),
            AnyValueEnum::StructValue(v) => v.get_name_or_default(),
            AnyValueEnum::VectorValue(v) => v.get_name_or_default(),
            AnyValueEnum::FunctionValue(v) => v.get_name_or_default(),
            AnyValueEnum::InstructionValue(v) => v.get_name_or_default(),
            AnyValueEnum::MetadataValue(v) => v.get_name_or_default(),
        }
    }
}
