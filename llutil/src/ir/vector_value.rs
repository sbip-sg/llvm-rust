//! Module provide additional utilities to handle LLVM `VectorValue`.

use inkwell::values::VectorValue;

/// Trait providing additional functions to handle `VectorValue`.
pub trait VectorExt {
    /// Get name of the `VectorValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `VectorExt` for `VectorValue`.
impl<'ctx> VectorExt for VectorValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-vector-name>".to_string(),
        }
    }
}
