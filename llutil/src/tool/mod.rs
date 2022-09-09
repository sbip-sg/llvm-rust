//! Module containing front-end external tools of Verazt

// Exporting sub-modules
pub mod clang;
pub mod llvm;
pub mod llvm_as;
pub mod llvm_dis;
pub mod llvm_opt;
pub mod rustc;
pub mod solang;
pub mod solc;

/// Executable file name of the Clang compiler
pub const CLANG: &str = "clang";

/// Executable file name of the LLVM assembler tool
pub const LLVM_AS: &str = "llvm-as";

/// Executable file name of the LLVM disassembler tool
pub const LLVM_DIS: &str = "llvm-dis";

/// Executable file name of the LLVM linking tool
pub const LLVM_LINK: &str = "llvm-link";

/// Executable file name of the LLVM optimization tool
pub const LLVM_OPT: &str = "opt";

/// Executable file name of the Solang compiler
pub const RUSTC: &str = "rustc";

/// Executable file name of the Solang compiler
pub const SOLANG: &str = "solang";

/// Executable file name of the Solc compiler
pub const SOLC: &str = "solc";

/// Required LLVM version
pub const LLVM_REQUIRED_VERSION: &str = ">=13.0.0";

/// Default compilation output directory
pub const OUTPUT_DIR: &str = "logs";

/// Executable file name of `Graphviz`
pub const DOT: &str = "dot";
