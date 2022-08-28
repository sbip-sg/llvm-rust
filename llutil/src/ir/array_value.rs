//! Module provide additional utilities to handle LLVM `ArrayValue`.

use inkwell::values::ArrayValue;

/// Trait providing additional functions to handle `ArrayValue`.
pub trait ArrayExt {
    /// Get name of the `ArrayValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `ArrayExt` for `ArrayValue`.
impl<'ctx> ArrayExt for ArrayValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-array-name>".to_string(),
        }
    }
}
