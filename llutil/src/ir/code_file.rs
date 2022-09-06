use std::fmt::{self, Display};

use inkwell::values::FunctionValue;

use crate::file::FileType;

use super::FunctionExt;

/// Data structure representing a code file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeFile {
    /// File name.
    pub file_name: String,

    /// File type.
    pub file_type: FileType,

    /// Original source code file name.
    pub source_file_name: Option<String>,

    /// Original source code file type.
    pub source_file_type: Option<FileType>,

    /// Names of entry functions of the current file.
    pub entry_point: EntryPoint,
}

/// Data structure representing entry points of a program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryPoint {
    /// All functions, including users and library functions.
    AllFunctions,

    /// All user functions.
    UserFunctions,

    /// Only main entry point functions.
    MainFunctions,
}

/// Implement methods for code file.
impl CodeFile {
    /// Constructor
    pub fn new(file_name: &str, entry_point: EntryPoint) -> Self {
        CodeFile {
            file_name: file_name.to_owned(),
            file_type: FileType::new(file_name),
            source_file_name: Some(file_name.to_owned()),
            source_file_type: Some(FileType::new(file_name)),
            entry_point,
        }
    }

    /// Check if the code file is an LLVM bitcode file
    pub fn is_llvm_bitcode(&self) -> bool {
        matches!(self.file_type, FileType::LLVMBC)
    }

    /// Check if the code file is original from C/C++.
    pub fn is_from_c_cpp(&self) -> bool {
        matches!(self.source_file_type, Some(FileType::CCpp))
    }

    /// Check if the code file is original from Solidity.
    pub fn is_from_solidity(&self) -> bool {
        matches!(self.source_file_type, Some(FileType::Solidity))
    }

    /// Check if a function is a library function of the current code file.
    pub fn check_library_function(&self, func: &FunctionValue) -> bool {
        self.check_c_cpp_library(func)
            || self.check_solidity_library(func)
            || self.check_solang_generated_library(func)
    }

    /// Check if a function is a C/C++ library of the current program.
    pub fn check_c_cpp_library(&self, func: &FunctionValue) -> bool {
        self.is_from_c_cpp() && func.is_c_library()
    }

    /// Check if a function is a Solidity library of the current program.
    pub fn check_solidity_library(&self, func: &FunctionValue) -> bool {
        self.is_from_solidity() && func.is_solidity_library()
    }

    /// Check if a function is a Solang-generated library of the current
    /// program.
    pub fn check_solang_generated_library(&self, func: &FunctionValue) -> bool {
        self.is_from_solidity() && func.is_solang_generated_library()
    }

    /// Check if a function is a C/C++ main function of the current program.
    pub fn check_c_cpp_main_function(&self, func: &FunctionValue) -> bool {
        self.is_from_c_cpp() && func.is_c_cpp_main_function()
    }

    /// Check if a function is a Solidity entry function of the current program.
    pub fn check_solidity_entry_function(&self, func: &FunctionValue) -> bool {
        self.is_from_solidity() && func.is_solidity_entry_function ()
    }
}

/// Implement trait `Display` for `CodeFile`.
impl Display for CodeFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_name)
    }
}
