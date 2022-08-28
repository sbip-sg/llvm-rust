//! Module handling code unit
use rutil::system;

/// Module containing extensions of supported code files.
pub mod ext {
    /// File extension of LLVM bitcode.
    pub const BC: &str = "bc";

    /// File extension of C programs.
    pub const C: &str = "c";

    /// File extension of C++ programs.
    pub const CPP: &str = "cpp";

    /// File extension of C++ programs.
    pub const CXX: &str = "cxx";

    /// File extension of EVM bytecode.
    pub const EVM: &str = "evm";

    /// File extension of C/C++ header files.
    pub const H: &str = "h";

    /// File extension of C++ header files.
    pub const HPP: &str = "hpp";

    /// File extension of C++ header files.
    pub const HXX: &str = "hxx";

    /// File extension of LLVM textual IR.
    pub const LL: &str = "ll";

    /// File extension of Rust files.
    pub const RS: &str = "rs";

    /// File extension of Solidity smart contracts.
    pub const SOL: &str = "sol";

    /// File extension of Yul intermediate code.
    pub const YUL: &str = "yul";

    /// File extension of results in running `Solang --emit`.
    pub const DOT: &str = "dot";
}

/// Data structure representing the supported file types.
///
/// REVIEW: rename this data structure to avoid conflict with Rust's `FileType`.
/// https://doc.rust-lang.org/std/fs/struct.FileType.html
#[remain::sorted]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    /// C/C++ and other C-family code files.
    CCpp,

    /// EMV bytecode files.
    EVMBC,

    /// LLVM bitcode files.
    LLVMBC,

    /// LLVM intermediate code files in textual format.
    LLVMIR,

    /// Rust source code files.
    Rust,

    /// Solidity source code files.
    Solidity,

    /// Unknown file type.
    Unknown,

    /// Yul intermediate code (IR) file.
    YulIR,
}

/// Implement methods for file types.
impl FileType {
    /// Constructor
    pub fn new(file_name: &str) -> Self {
        match system::get_file_ext(file_name) {
            Some(
                ext::C | ext::CPP | ext::CXX | ext::H | ext::HPP | ext::HXX,
            ) => FileType::CCpp,
            Some(ext::SOL) => FileType::Solidity,
            Some(ext::BC) => FileType::LLVMBC,
            Some(ext::LL) => FileType::LLVMIR,
            Some(ext::EVM) => FileType::EVMBC,
            Some(ext::YUL) => FileType::YulIR,
            _ => FileType::Unknown,
        }
    }

    /// Check if the current file is a C/C++ file.
    pub fn is_c_cpp_code(&self) -> bool {
        matches!(self, FileType::CCpp)
    }

    /// Check if the current file is a C/C++ file.
    pub fn is_solidity_code(&self) -> bool {
        matches!(self, FileType::Solidity)
    }
}
