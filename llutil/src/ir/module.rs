//! Module provide additional utilities to handle LLVM `Module`.

use inkwell::module::Module;

/// Trait provide utilities to handle `Module`.
pub trait ModuleExt {
    /// Get name of the module or return a default name.
    fn get_name_or_default(&self) -> String;
}

/// Implement the trait `ModuleExt` for `Module`.
impl<'ctx> ModuleExt for Module<'ctx> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<unknown-module>".to_string(),
        }
    }
}
