//! Module provide additional utilities to handle LLVM `Module`.

use inkwell::module::Module;

use crate::file::FileType;

/// Trait provide utilities to handle `Module`.
pub trait ModuleExt {
    /// Get module name.
    fn get_name2(&self) -> Option<String>;

    /// Get name of the module or return a default name.
    fn get_name_or_default(&self) -> String;

    /// Check if the current module is originally from a C/C++ file.
    fn is_originally_from_c_cpp(&self) -> bool;

    /// Check if the current module is originally from a Solidity file.
    fn is_originally_from_solidity(&self) -> bool;
}

/// Implement the trait `ModuleExt` for `Module`.
impl<'ctx> ModuleExt for Module<'ctx> {
    fn get_name2(&self) -> Option<String> {
        match self.get_name().to_str() {
            Ok(name) => Some(name.to_string()),
            _ => None,
        }
    }

    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<unknown-module>".to_string(),
        }
    }

    fn is_originally_from_c_cpp(&self) -> bool {
        match self.get_name2() {
            None => false,
            Some(name) => {
                let filetype = FileType::new(&name);
                filetype.is_c_cpp_code()
            }
        }
    }

    fn is_originally_from_solidity(&self) -> bool {
        match self.get_name2() {
            None => false,
            Some(name) => {
                let filetype = FileType::new(&name);
                filetype.is_solidity_code()
            }
        }
    }
}
