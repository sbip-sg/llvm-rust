//! Module provide additional utilities to handle LLVM `IntValue`.

use inkwell::values::IntValue;

/// Trait providing additional functions to handle `IntValue`.
pub trait VectorExt {
    /// Get name of the `IntValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `VectorExt` for `IntValue`.
impl<'ctx> VectorExt for IntValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-vector-name>".to_string(),
        }
    }
}
