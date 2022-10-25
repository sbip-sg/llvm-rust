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
pub mod solana;

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

/// Executable file name of the Rustc compiler
pub const RUSTC: &str = "rustc";

/// Executable file name of the Solang compiler
pub const SOLANG: &str = "solang";

/// Executable file name of the Solc compiler
pub const SOLC: &str = "solc";

/// Executable file name of the cargo 
pub const CARGO: &str = "cargo";

/// Executable file name of the cargo-build-sbf for check version
pub const CARGO_BUILD_SBF: &str = "cargo-build-sbf";

/// Executable file name of the cargo-build-bpf for compiler of solana
pub const CARGO_BUILD_BPF: &str = "cargo-build-bpf";

/// Required LLVM version
pub const LLVM_REQUIRED_VERSION: &str = ">=13.0.0";

/// Default compilation output directory
pub const OUTPUT_DIR: &str = "logs";

/// Compilation output directory for solana
pub const OUTPUT_TARGET_DIR: &str = "target";

/// Manifest for solana
pub const CARGO_TOML: &str = "Cargo.toml";
