//! Module provide additional utilities to handle LLVM `FunctionValue`.

use crate::ir::builtin;
use inkwell::values::{AnyValue, FunctionValue};
use rutil::string::StringUtil;

/// Extra utility functions to handle `FunctionValue`
pub trait FunctionExt {
    /// Check if the function is declared but not defined.
    fn is_only_declared(&self) -> bool;

    /// Print function header including names and parameters to `String`.
    fn print_header(&self) -> String;

    /// Print the `FunctionValue` to string in a pretty format.
    fn print_pretty(&self) -> String;

    // /// Check if the current function is a library function.
    // fn is_library_function(&self, file: &CodeFile) -> bool;

    /// Check if the current function is a C library function.
    fn is_c_library_function(&self) -> bool;

    // /// Check if the current function is a C main function.
    // fn is_c_main_function(&self, file: &CodeFile) -> bool;

    // /// Check if the current function is a Solidity library  function.
    // fn is_solidity_library_function(&self, file: &CodeFile) -> bool;

    // /// Check if the current function is a Solidity entry function.
    // fn is_solidity_entry_function(&self, file: &CodeFile) -> bool;

    /// Check if the current function is an LLVM library function.
    fn is_llvm_intrinsic_function(&self) -> bool;

    /// Check if the current function is a Verazt lirbary function.
    fn is_verazt_library_function(&self) -> bool;
}

impl<'a> FunctionExt for FunctionValue<'a> {
    fn print_header(&self) -> String {
        let params = self
            .get_param_iter()
            .map(|p| p.print_to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{}({})", self.get_name_or_default(), params)
    }

    fn is_only_declared(&self) -> bool {
        // A function is declared but not defined if its body is empty.
        self.count_basic_blocks() == 0
    }

    fn print_pretty(&self) -> String {
        let params = self
            .get_param_iter()
            .map(|p| p.print_to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let mut res =
            formati!(0, "Function: {}({})", self.get_name_or_default(), params)
                .indent_tail_lines(2);

        let blocks = self
            .get_basic_blocks()
            .into_iter()
            .map(|blk| blk.print_to_string().indent(2))
            .collect::<Vec<String>>()
            .join("\n\n");

        if blocks.is_empty() {
            res += "\n  (Empty body)"
        } else {
            res = res + "\n" + &blocks;
        }

        res
    }

    // fn is_library_function(&self, file: &CodeFile) -> bool {
    //     self.is_c_library_function(file)
    //         || self.is_solidity_library_function(file)
    //         || self.is_verazt_library_function()
    // }

    fn is_c_library_function(&self) -> bool {
        // file.is_originally_from_c_cpp()
        //     && builtin::is_c_library_function(&self.get_name_or_default())
        true
    }

    // fn is_c_main_function(&self, file: &CodeFile) -> bool {
    //     file.is_originally_from_c_cpp()
    //         && builtin::is_c_main_function(&self.get_name_or_default())
    // }

    // fn is_solidity_library_function(&self, file: &CodeFile) -> bool {
    //     file.is_originally_from_solidity()
    //         && builtin::is_solidity_library_function(
    //             &self.get_name_or_default(),
    //         )
    // }

    // fn is_solidity_entry_function(&self, file: &CodeFile) -> bool {
    //     file.is_originally_from_solidity()
    //         && !builtin::is_c_library_function(&self.get_name_or_default())
    // }

    fn is_llvm_intrinsic_function(&self) -> bool {
        builtin::is_llvm_intrinsic_function(&self.get_name_or_default())
    }

    fn is_verazt_library_function(&self) -> bool {
        builtin::is_verazt_library_function(&self.get_name_or_default())
    }
}

/// Trait of utilities for a `Vector` of `FunctionValue`.
pub trait FunctionVec {
    /// Get names of all functions
    fn get_names(&self) -> Vec<String>;

    /// Print function names to String.
    fn print_names(&self) -> String;
}

impl<'a> FunctionVec for Vec<FunctionValue<'a>> {
    fn get_names(&self) -> Vec<String> {
        self.iter()
            .map(|f| f.get_name_or_default())
            .collect::<Vec<String>>()
    }

    fn print_names(&self) -> String {
        self.get_names().join(", ")
    }
}

/// Trait of utilities for an `Option` of `FunctionValue`.
pub trait FunctionOption {
    /// Get name of the current function, if any.
    fn get_name_or_default(&self) -> String;
}

impl<'a> FunctionOption for Option<FunctionValue<'a>> {
    /// Get names
    fn get_name_or_default(&self) -> String {
        match self {
            None => "<None>".to_owned(),
            Some(func) => func.get_name_or_default(),
        }
    }
}
