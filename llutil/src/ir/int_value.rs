//! Module provide additional utilities to handle LLVM `IntValue`.

use inkwell::values::IntValue;

/// Trait providing additional functions to handle `IntValue`.
pub trait IntExt {
    /// Get name of the `IntValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `IntExt` for `IntValue`.
impl<'ctx> IntExt for IntValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        if let Some(i) = self.get_sign_extended_constant() {
            i.to_string()
        } else if let Some(i) = self.get_zero_extended_constant() {
            i.to_string()
        } else if let Some(i) = self.get_big_int_constant() {
            i.to_string()
        } else {
            match self.get_name().to_str() {
                Ok(name) => name.to_string(),
                _ => "<empty-int-value-name>".to_string(),
            }
        }
    }
}
