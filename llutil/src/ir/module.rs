//! Module provide additional utilities to handle LLVM `Module`.

use inkwell::module::Module;

use crate::file::FileType;

/// Trait provide utilities to handle `Module`.
pub trait ModuleExt {
    /// Get module name.
    fn get_module_name(&self) -> Option<String>;

    /// Check if the current module is originally from a C/C++ file.
    fn is_origially_from_c_cpp(&self) -> bool;

    /// Check if the current module is originally from a Solidity file.
    fn is_originally_from_solidity(&self) -> bool;
}

/// Implement the trait `ModuleExt` for `Module`.
impl<'ctx> ModuleExt for Module<'ctx> {
    fn get_module_name(&self) -> Option<String> {
        match self.get_name().to_str() {
            Ok(name) => Some(name.to_string()),
            _ => None,
        }
    }

    fn is_origially_from_c_cpp(&self) -> bool {
        match self.get_module_name() {
            None => false,
            Some(name) => {
                let filetype = FileType::new(&name);
                filetype.is_c_cpp_code()
            }
        }
    }

    fn is_originally_from_solidity(&self) -> bool {
        match self.get_module_name() {
            None => false,
            Some(name) => {
                let filetype = FileType::new(&name);
                filetype.is_solidity_code()
            }
        }
    }
}
