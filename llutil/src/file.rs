//! Module handling code unit

use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::{fmt, path::Path};

use inkwell::{context::Context, module::Module};

use crate::instrument::bug_annot::instrument_bug_annotations;
use crate::normalize;
use crate::tool::compiler::{Compiler, CompilerOptions};
use crate::{
    global,
    tool::{clang, llvm_as, llvm_dis, llvm_opt, rustc, solang, solc},
};
use rutil::{cli::CoreOptions, system};

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

/// Data structure representing a file structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeFile {
    /// File name.
    pub file_name: String,

    /// File type.
    pub file_type: FileType,

    /// Names of entry functions of the current file.
    pub entry_point: EntryPoint,

    /// Original file name.
    pub original_file_name: Option<String>,

    /// Original file type.
    pub original_file_type: Option<FileType>,
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
}

/// Implement methods for code file.
impl CodeFile {
    /// Constructor
    pub fn new(file_name: &str, entry_point: EntryPoint) -> Self {
        let file_type = FileType::new(file_name);

        CodeFile {
            file_name: file_name.to_owned(),
            file_type,
            entry_point,
            original_file_name: None,
            original_file_type: None,
        }
    }

    /// Constructor
    pub fn derive_from_file(file_name: &str, orig_file: &CodeFile) -> CodeFile {
        let mut file =
            CodeFile::new(file_name, orig_file.entry_point.to_owned());

        file.original_file_name = match &orig_file.original_file_name {
            None => Some(orig_file.file_name.to_owned()),
            Some(fname) => Some(fname.to_owned()),
        };

        file.original_file_type = match &orig_file.original_file_type {
            None => Some(orig_file.file_type.to_owned()),
            Some(ftype) => Some(ftype.to_owned()),
        };

        file
    }

    /// Check if the compiled file is an LLVM bitcode file
    pub fn is_llvm_bitcode(&self) -> bool {
        matches!(self.file_type, FileType::LLVMBC)
    }

    /// Check if the compiled file is originally from a C/C++ file.
    pub fn is_originally_from_c_cpp(&self) -> bool {
        matches!(self.original_file_type, Some(FileType::CCpp))
    }

    /// Check if the compiled file is originally from a Solidity contract.
    pub fn is_originally_from_solidity(&self) -> bool {
        matches!(self.original_file_type, Some(FileType::Solidity))
    }

    /// Compiling input files into LLVM bitcode.
    /// Output: a list of generated bitcode files (*.bc).
    pub fn compile(&self, opts: &CompilerOptions) -> Vec<Self> {
        let filename = &self.file_name;
        let compiler = &opts.compiler;

        debug!("Compiling file: {}", self.file_name);

        let outputs = match compiler {
            Compiler::Clang => clang::compile(
                self,
                &opts.clang_options,
                &opts.include_dirs,
                &opts.include_files,
            ),

            Compiler::Rustc => rustc::compile(self, &opts.clang_options),
            Compiler::Solang => solang::compile(self, &opts.solang_options),
            Compiler::Solc => {
                solc::compile_to_evm_bytecode(self, &opts.solc_options)
            }
            Compiler::Unknown => match self.file_type {
                FileType::CCpp => clang::compile(
                    self,
                    &opts.clang_options,
                    &opts.include_dirs,
                    &opts.include_files,
                ),
                FileType::EVMBC => panic!("Need to compile: {}", filename),
                FileType::LLVMBC => panic!("Need to compile: {}", filename),
                FileType::LLVMIR => llvm_as::assemble(self),
                FileType::Rust => rustc::compile(self, &opts.rustc_options),
                FileType::Solidity => {
                    solang::compile(self, &opts.solang_options)
                }
                FileType::Unknown => panic!("Need to compile: {}", filename),
                FileType::YulIR => panic!("Need to compile: {}", filename),
            },
        };

        // Disassemble compiled files
        unsafe {
            if global::DEBUG_MODE {
                for output in &outputs {
                    if let FileType::LLVMBC = output.file_type {
                        llvm_dis::disassemble(&output.file_name);
                    }
                }
            }
        }

        // Print compiled files to `stdout`
        if opts.print_compiled_prog {
            for file in &outputs {
                debug!("===== COMPILED PROGRAM =====");
                match file.file_type {
                    FileType::LLVMBC => print_bitcode(&file.file_name),
                    _ => {
                        fixme!("Print compiled program!")
                    }
                }
            }
        }

        outputs
    }

    /// Instrument intermediate code to compiled file
    pub fn instrument_code(
        self,
        copts: &CoreOptions,
        source_file: &str,
    ) -> Self {
        debug!("Instrument code to file: {}", self.file_name);

        if let FileType::LLVMBC = self.file_type {
            let ctx = Context::create();
            let module =
                match Module::parse_bitcode_from_path(&self.file_name, &ctx) {
                    Ok(module) => module,
                    _ => panic!(
                        "Failed to parse LLVM bitcode: {}",
                        &self.file_name
                    ),
                };
            instrument_bug_annotations(source_file, &module, &ctx);

            // Write to bitcode file
            let input_file_path = Path::new(&self.file_name);
            let file_stem_name = input_file_path
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap_or("");
            let parent_dir =
                input_file_path.parent().unwrap_or_else(|| Path::new(""));

            let out_file_path =
                parent_dir.join(file_stem_name.to_owned() + ".instr.bc");
            let out_file_name = out_file_path.to_str().unwrap();

            fs::remove_file(out_file_name).unwrap_or(());
            module.write_bitcode_to_path(Path::new(out_file_name));

            // Disassemble instrumented program
            unsafe {
                if global::DEBUG_MODE {
                    llvm_dis::disassemble(out_file_name);
                }
            }

            let out_file = CodeFile::derive_from_file(out_file_name, &self);

            // Print instrumented files to `stdout`
            if copts.print_instrumented_prog {
                debug!("===== INSTRUMENTED PROGRAM =====");
                match out_file.file_type {
                    FileType::LLVMBC => print_bitcode(&out_file.file_name),
                    _ => {
                        fixme!("Print instrumented program!")
                    }
                }
            }

            return out_file;
        }

        self
    }

    /// Optimize an intermediate code file by
    pub fn optimize(self, copts: &CoreOptions) -> Self {
        debug!("Optimizing file: {}", self.file_name);

        // Optimize file
        let out_file = match self.file_type {
            FileType::LLVMBC => {
                let out_file = llvm_opt::optimize(&self);

                // Disassemble optimized program
                unsafe {
                    if global::DEBUG_MODE {
                        llvm_dis::disassemble(&out_file.file_name);
                    }
                }

                out_file
            }

            _ => self,
        };

        // Print optimized files to `stdout`
        if copts.print_optimized_prog {
            debug!("===== OPTIMIZED PROGRAM =====");
            match out_file.file_type {
                FileType::LLVMBC => print_bitcode(&out_file.file_name),
                _ => {
                    fixme!("Print optimized program!")
                }
            }
        }

        out_file
    }

    /// Normalize an intermediate code file by
    pub fn normalize(self, opts: &CoreOptions) -> Self {
        debug!("Normalizing file: {}", self.file_name);

        //  Normalize file
        let out_file = match self.file_type {
            FileType::LLVMBC => {
                let out_file = normalize::normalize_bitcode_file(&self);

                // Disassemble optimized program
                unsafe {
                    if global::DEBUG_MODE {
                        llvm_dis::disassemble(&out_file.file_name);
                    }
                }

                return out_file;
            }

            _ => self,
        };

        // Print optimized files to `stdout`
        if opts.print_optimized_prog {
            debug!("===== OPTIMIZED PROGRAM =====");
            match out_file.file_type {
                FileType::LLVMBC => print_bitcode(&out_file.file_name),
                _ => {
                    fixme!("Print optimized program!")
                }
            }
        }

        out_file
    }
}

// Implement trait Display
impl Display for CodeFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.file_name)
    }
}

/// Print an LLVM bitcode file to string
pub fn print_bitcode(filename: &str) {
    let context = Context::create();
    let output = Module::parse_bitcode_from_path(filename, &context);

    match output {
        Err(_) => warning!("Failed to parse bitcode file: {}", filename),
        Ok(module) => println!("{}", module.print_to_string()),
    }
}
