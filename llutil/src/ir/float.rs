//! Module provide additional utilities to handle LLVM `FloatValue`.

use inkwell::values::FloatValue;

/// Trait providing additional functions to handle `FloatValue`.
pub trait FloatExt {
    /// Get name of the `FloatValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `FloatExt` for `FloatValue`.
impl<'ctx> FloatExt for FloatValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-float-name>".to_string(),
        }
    }
}
