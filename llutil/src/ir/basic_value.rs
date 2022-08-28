//! Module provide additional utilities to handle LLVM `BasicValueEnum`.

use inkwell::values::BasicValueEnum;

use super::{ArrayExt, FloatExt, IntExt, PointerExt, StructExt, VectorExt};

/// Trait providing additional functions to handle `BasicValueEnum`.
pub trait BasicValueExt {
    /// Get name of the `BasicValueEnum` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `BasicValueExt` for `BasicValueEnum`.
impl<'ctx> BasicValueExt for BasicValueEnum<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self {
            BasicValueEnum::ArrayValue(v) => v.get_name_or_default(),
            BasicValueEnum::IntValue(v) => v.get_name_or_default(),
            BasicValueEnum::FloatValue(v) => v.get_name_or_default(),
            BasicValueEnum::PointerValue(v) => v.get_name_or_default(),
            BasicValueEnum::StructValue(v) => v.get_name_or_default(),
            BasicValueEnum::VectorValue(v) => v.get_name_or_default(),
        }
    }
}
