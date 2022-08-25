//! Main module for LLVM bitcode normalization

use inkwell::context::Context;
use inkwell::module::Module;
use std::{ffi::OsStr, path::Path};

use crate::file::CodeFile;

// Exporting sub-modules
pub mod rename;
pub mod simplify;
pub mod transform;

/// Normalize an LLVM bitcode module.
///
/// REVIEW: turn this normalization into trait.
pub fn normalize_bitcode_module(file: CodeFile, module: Module) -> Module {
    let filename = module.get_name_or_default();
    debug!("Normalizing module: {}", filename);

    // Simplify the bitcode module by Verazt's passes
    simplify::simplify_module(&file, &module);

    // Transform the bitcode module by LLVM passes
    transform::transform_module(&file, &module);

    // Finally, rename items in bitcode to make it uniform
    rename::rename_basic_blocks_and_values(&module);

    // return
    module
}

/// Normalize an LLVM bitcode file.
pub fn normalize_bitcode_file(file: &CodeFile) -> CodeFile {
    let filename = &file.file_name;
    let context = Context::create();
    let module = match Module::parse_bitcode_from_path(filename, &context) {
        Ok(module) => module,
        _ => panic!("Failed to parse LLVM bitcode file: {}", filename),
    };

    let output_module = normalize_bitcode_module(file.to_owned(), module);

    // write bitcode to file
    let output_path = {
        let file_path = Path::new(filename.as_str());
        let file_stem = match file_path.file_stem().and_then(OsStr::to_str) {
            Some(name) => name,
            None => panic!("Invalid input file name: {}", file_path.display()),
        };

        let parent_dir = file_path.parent().unwrap_or_else(|| Path::new(""));

        parent_dir.join(file_stem.to_owned() + ".norm.bc")
    };

    let output_filename = match output_path.to_str() {
        Some(name) => name,
        None => panic!("Invalid output file name: {}", output_path.display()),
    };

    output_module.set_name(output_filename);
    output_module.write_bitcode_to_path(&output_path);

    debug!("Output normalized file: {}", output_filename);

    CodeFile::derive_from_file(output_filename, file)
}
