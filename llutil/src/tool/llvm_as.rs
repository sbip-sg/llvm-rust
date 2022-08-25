//! Module invoking the LLVM assembler tool for textual IR (*.ll) files.

use regex::Regex;
use semver::{Version, VersionReq};
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::file::CodeFile;
use crate::tool;
use crate::tool::OUTPUT_DIR;
use rutil::report;
use rutil::system;

use super::LLVM_REQUIRED_VERSION;

/// Check path of the LLVM assembler tool (llvm-as)
fn check_llvm_assembler_path() {
    match system::path_of_command_from_env(tool::LLVM_AS) {
        Ok(path) => debug!("llvm-as path: {}", path),
        Err(_) => panic!("llvm-as path not found: {}!", tool::LLVM_AS),
    }
}

/// Check version of the LLVM assembler tool (llvm-as)
fn check_llvm_assembler_version() {
    let llvm_as_output =
        Command::new(tool::LLVM_AS).args(&["--version"]).output();
    match llvm_as_output {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"version (\d+\.\d+\.\d+)").unwrap();
            let llvm_as_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            let llvm_as_ver = match Version::parse(llvm_as_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("llvm-as version not found: {}", msg),
            };
            let llvm_ver = match VersionReq::parse(LLVM_REQUIRED_VERSION) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("LLVM required version is invalid: {}", msg)
                }
            };
            if !llvm_ver.matches(&llvm_as_ver) {
                panic!(
                    "Expect llvm-as version {} but found: {}",
                    llvm_ver, llvm_as_ver
                );
            }
        }

        Err(_) => {
            panic!(
                "Check llvm-as version: command not found: {}",
                tool::LLVM_AS
            );
        }
    }
}

/// Check settings of the LLVM assembler tool (llvm-as)
pub fn check_llvm_assembler_settings() {
    check_llvm_assembler_path();
    check_llvm_assembler_version()
}

/// Compile LLVM IR programs and return the output bitcode file name.
pub fn assemble(file: &CodeFile) -> Vec<CodeFile> {
    let filename = &file.file_name;
    let filepath = Path::new(filename);
    let file_stem_name =
        filepath.file_stem().and_then(OsStr::to_str).unwrap_or("");
    let parent_dir = filepath.parent().unwrap_or_else(|| Path::new(""));

    // prepare output folder
    let log_dir = parent_dir.join(OUTPUT_DIR);
    let output_file_path = log_dir.join(file_stem_name.to_owned() + ".bc");
    let output_file_name = output_file_path.to_str().unwrap();
    fs::remove_file(output_file_name).unwrap_or(());
    fs::create_dir_all(log_dir.to_str().unwrap()).unwrap_or(());

    let llvm_as_args =
        filename.to_owned() + format!(" -o {}", output_file_name).as_str();

    // debug!("Running command: {} {}", tool::LLVM_AS, llvm_as_args);

    let llvm_as_output = Command::new(tool::LLVM_AS)
        .args(llvm_as_args.split_whitespace())
        .output()
        .unwrap();

    if !llvm_as_output.status.success() {
        let error_msg = String::from_utf8(llvm_as_output.stderr.to_vec())
            .expect("llvm-as: unknown error!");
        report::print_message("llvm-as error message:", error_msg.as_str());
        panic!("Failed to compile: {}", filename);
    }

    let output_file = CodeFile::derive_from_file(output_file_name, file);
    vec![output_file]
}
