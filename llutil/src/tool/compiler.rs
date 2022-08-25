//! Module to invoke different compilers for different kind of input files.

use std::fmt::{self, Display};

/// Default compilation output directory
pub const OUTPUT_DIR: &str = "logs";

/// List of supported compilers
#[remain::sorted]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Compiler {
    /// Clang compiler
    Clang,
    /// Rustc compiler
    Rustc,
    /// Solang compiler
    Solang,
    /// Solc compiler
    Solc,
    /// Unknown compiler
    Unknown,
}

/// Data structure capturing compiler options.
#[remain::sorted]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompilerOptions<'a> {
    /// Option to pass specific options to Clang.
    pub clang_options: Vec<&'a str>,

    /// Option to choose a default compiler.
    pub compiler: Compiler,

    /// Option to capture directory paths containing supporting libraries.
    pub include_dirs: Vec<&'a str>,

    /// Option to capture file paths of supporting libraries.
    pub include_files: Vec<&'a str>,

    /// Option to print the compiled program to `stdout`.
    pub print_compiled_prog: bool,

    /// Option to pass specific options to Rustc.
    pub rustc_options: Vec<&'a str>,

    /// Option to pass specific options to Solang.
    pub solang_options: Vec<&'a str>,

    /// Option to pass specific options to Solc.
    pub solc_options: Vec<&'a str>,
}

/// Implement methods for `CompilerOptions`.
impl<'a> CompilerOptions<'a> {
    // /// Constructor
    // pub fn from_core_option(core_opts: &'a CoreOptions<'a>) -> Self {
    //     CompilerOptions {
    //         clang_options: core_opts.clang_options.to_owned(),
    //         include_dirs: core_opts.include_dirs.to_owned(),
    //         include_files: core_opts.include_files.to_owned(),
    //         print_compiled_prog: core_opts.print_compiled_prog,
    //         rustc_options: core_opts.rustc_options.to_owned(),
    //         solang_options: core_opts.solang_options.to_owned(),
    //         solc_options: core_opts.solc_options.to_owned(),
    //         compiler: Compiler::Unknown,
    //     }
    // }
}

// Implement the trait `Display` for `Compiler`.
impl Display for Compiler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Compiler::Clang => write!(f, "Clang"),
            Compiler::Rustc => write!(f, "Rustc"),
            Compiler::Solang => write!(f, "Solang"),
            Compiler::Solc => write!(f, "Solc"),
            Compiler::Unknown => write!(f, "<UnknownCompiler>"),
        }
    }
}
