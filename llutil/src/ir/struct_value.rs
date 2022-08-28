//! Module provide additional utilities to handle LLVM `StructValue`.

use inkwell::values::StructValue;

/// Trait providing additional functions to handle `StructValue`.
pub trait StructExt {
    /// Get name of the `StructValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `StructExt` for `StructValue`.
impl<'ctx> StructExt for StructValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-struct-name>".to_string(),
        }
    }
}
