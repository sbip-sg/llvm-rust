//! Module providing additional utilities to handle LLVM `FunctionValue`.

use std::collections::HashSet;

use crate::ir::builtin;
use inkwell::{
    module::Module,
    values::{AnyValue, FunctionValue, GlobalValue},
};
use rutil::string::StringExt;

use super::{basic_block::BasicBlockExt, module::ModuleExt};

/// Trait providing additional functions to handle `FunctionValue`
pub trait FunctionExt {
    /// Get name of the `FunctionValue` or return a default name.
    fn get_name_or_default(&self) -> String;

    /// Check if the function is declared but not defined.
    fn is_only_declared(&self) -> bool;

    /// Print function header including names and parameters to `String`.
    fn print_header(&self) -> String;

    /// Print the `FunctionValue` to string in a pretty format.
    fn print_pretty(&self) -> String;

    /// Check if the current function is a library function.
    ///
    /// NOTE: currently need to pass `module` as a parameter since there is a
    /// bug in Inkwell that calling to `FunctionValue::get_parent` will make the
    /// program crash. Remove this parameter once Inkwell are fixed.
    fn is_library_function(&self, module: &Module) -> bool;

    /// Check if the current function is a C library function.
    ///
    /// NOTE: currently need to pass `module` as a parameter since there is a
    /// bug in Inkwell that calling to `FunctionValue::get_parent` will make the
    /// program crash. Remove this parameter once Inkwell are fixed.
    fn is_c_library_function(&self, module: &Module) -> bool;

    /// Check if the current function is a Solidity library  function.
    ///
    /// NOTE: currently need to pass `module` as a parameter since there is a
    /// bug in Inkwell that calling to `FunctionValue::get_parent` will make the
    /// program crash. Remove this parameter once Inkwell are fixed.
    fn is_solidity_library_function(&self, module: &Module) -> bool;

    /// Check if the current function is a Solang-generated function.
    ///
    /// NOTE: currently need to pass `module` as a parameter since there is a
    /// bug in Inkwell that calling to `FunctionValue::get_parent` will make the
    /// program crash. Remove this parameter once Inkwell are fixed.
    fn is_solidity_solang_generated_function(&self, module: &Module) -> bool;

    /// Check if the current function is an LLVM library function.
    fn is_llvm_intrinsic_function(&self) -> bool;

    /// Check if the current function is an assertion checking function.
    fn is_assertion_checking_function(&self) -> bool;

    /// Check if the current function is a C main function.
    ///
    /// NOTE: currently need to pass `module` as a parameter since there is a
    /// bug in Inkwell that calling to `FunctionValue::get_parent` will make the
    /// program crash. Remove this parameter once Inkwell are fixed.
    fn is_c_main_function(&self, module: &Module) -> bool;

    /// Check if the current function is a Solidity entry function.
    ///
    /// NOTE: currently need to pass `module` as a parameter since there is a
    /// bug in Inkwell that calling to `FunctionValue::get_parent` will make the
    /// program crash. Remove this parameter once Inkwell are fixed.
    fn is_solidity_entry_function(&self, module: &Module) -> bool;
}

impl<'a> FunctionExt for FunctionValue<'a> {
    fn get_name_or_default(&self) -> String {
        match self.get_name().to_str() {
            Ok(name) => name.to_string(),
            _ => "<empty-function-name>".to_string(),
        }
    }

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
            .map(|blk| blk.print_pretty().indent(2))
            .collect::<Vec<String>>()
            .join("\n\n");

        if blocks.is_empty() {
            res += "\n  (Empty body)"
        } else {
            res = res + "\n" + &blocks;
        }

        res
    }

    fn is_library_function(&self, module: &Module) -> bool {
        self.is_c_library_function(module)
            || self.is_solidity_library_function(module)
            || self.is_assertion_checking_function()
    }

    fn is_c_library_function(&self, module: &Module) -> bool {
        module.is_originally_from_c_cpp()
            && builtin::is_c_library_function(&self.get_name_or_default())
    }

    fn is_solidity_library_function(&self, module: &Module) -> bool {
        module.is_originally_from_solidity()
            && builtin::is_solidity_library_function(
                &self.get_name_or_default(),
            )
    }

    fn is_solidity_solang_generated_function(&self, module: &Module) -> bool {
        module.is_originally_from_solidity()
            // A Solang generated function will not contain the string "::"
            && !self.get_name_or_default().contains("::")
    }

    fn is_llvm_intrinsic_function(&self) -> bool {
        builtin::is_llvm_intrinsic_function(&self.get_name_or_default())
    }

    fn is_assertion_checking_function(&self) -> bool {
        builtin::is_assertio_checking_function(&self.get_name_or_default())
    }

    fn is_c_main_function(&self, module: &Module) -> bool {
        module.is_originally_from_c_cpp()
            && builtin::is_c_main_function(&self.get_name_or_default())
    }

    fn is_solidity_entry_function(&self, module: &Module) -> bool {
        module.is_originally_from_solidity()
            && !builtin::is_solidity_library_function(
                &self.get_name_or_default(),
            )
    }
}

/// Trait of utilities for a `Vector` of `GlobalValue`.
pub trait GlobalVec {
    /// Print global variables to String.
    fn print_to_string(&self) -> String;
}

impl<'a> GlobalVec for Vec<GlobalValue<'a>> {
    fn print_to_string(&self) -> String {
        let res = self
            .iter()
            .map(|g| formati!(2, "{}", g).indent_tail_lines(2))
            .collect::<Vec<String>>()
            .join("\n");
        ite!(res.is_empty(), "[]".to_string(), "\n".to_string() + &res)
    }
}

/// Trait of utilities for a collection (`Vector`, `HashSet`, etc) of
/// `FunctionValue`.
pub trait Functions {
    /// Get names of all functions
    ///
    /// This function needs to be implemented by each data structure.
    fn get_names(&self) -> Vec<String>;

    /// Print function names to String.
    fn print_names(&self) -> String {
        self.get_names().join(", ")
    }

    /// Print function names to a list.
    fn print_bulleted_names(&self) -> String {
        let res = self
            .get_names()
            .iter()
            .map(|f| formatp!(0, 0, "- ", "{}", f).indent_tail_lines(2))
            .collect::<Vec<String>>()
            .join("\n");

        ite!(res.is_empty(), "[]".to_string(), "\n".to_string() + &res)
    }
}

/// Implement trait `Functions` for `Vec<&FunctionValue>`.
impl<'a> Functions for Vec<&FunctionValue<'a>> {
    fn get_names(&self) -> Vec<String> {
        self.iter()
            .map(|f| f.get_name_or_default())
            .collect::<Vec<String>>()
    }
}

/// Implement trait `Functions` for `Vec<FunctionValue>`.
impl<'a> Functions for Vec<FunctionValue<'a>> {
    fn get_names(&self) -> Vec<String> {
        self.iter()
            .map(|f| f.get_name_or_default())
            .collect::<Vec<String>>()
    }
}

/// Implement trait `Functions` for `HashSet<FunctionValue>`.
impl<'a> Functions for HashSet<FunctionValue<'a>> {
    fn get_names(&self) -> Vec<String> {
        let mut names = self
            .iter()
            .map(|f| f.get_name_or_default())
            .collect::<Vec<String>>();
        names.sort();
        names
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
