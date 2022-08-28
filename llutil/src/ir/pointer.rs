//! Module provide additional utilities to handle LLVM `PointerValue`.

use inkwell::values::PointerValue;

/// Trait providing additional functions to handle `PointerValue`.
pub trait PointerExt {
    /// Get name of the `PointerValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `PointerExt` for `PointerValue`.
impl<'ctx> PointerExt for PointerValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-pointer-name>".to_string(),
        }
    }
}
