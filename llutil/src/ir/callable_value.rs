//! Module provide additional utilities to handle LLVM `CallabelValue`.

use inkwell::values::CallableValue;

use super::{FunctionExt, PointerExt};

/// Trait providing additional functions to handle `CallableValue`.
pub trait CallableExt {
    /// Get name of the `CallableValue` or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `CallableExt` for `CallableValue`.
impl<'ctx> CallableExt for CallableValue<'ctx> {
    fn get_name_or_default(&self) -> String {
        if let Some(ptr) = self.as_pointer_value() {
            ptr.get_name_or_default()
        } else if let Some(func) = self.as_function_value() {
            func.get_name_or_default()
        } else {
            "<empty-callable-name>".to_owned()
        }
    }
}
