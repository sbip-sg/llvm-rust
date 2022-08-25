//! Module invoking the LLVM disassembler tool for bitcode (*.bc) files.

use regex::Regex;
use semver::{Version, VersionReq};
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::tool;
use rutil::report;
use rutil::system;

use super::LLVM_REQUIRED_VERSION;

/// Check path of the LLVM disassembler tool (llvm-dis)
fn check_llvm_disassembler_path() {
    match system::path_of_command_from_env(tool::LLVM_DIS) {
        Ok(path) => debug!("llvm-dis path: {}", path),
        Err(_) => panic!("llvm-dis path not found: {}!", tool::LLVM_DIS),
    }
}

/// Check version of the LLVM disassembler tool (llvm-dis)
fn check_llvm_disassembler_version() {
    let llvm_dis_output =
        Command::new(tool::LLVM_DIS).args(&["--version"]).output();
    match llvm_dis_output {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"version (\d+\.\d+\.\d+)").unwrap();
            let llvm_dis_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            let llvm_dis_ver = match Version::parse(llvm_dis_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("llvm-dis version not found: {}", msg),
            };
            let llvm_ver = match VersionReq::parse(LLVM_REQUIRED_VERSION) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("LLVM required version invalid: {}", msg)
                }
            };
            if !llvm_ver.matches(&llvm_dis_ver) {
                panic!(
                    "Expect llvm-dis version {} but found: {}",
                    llvm_ver, llvm_dis_ver
                );
            }
        }

        Err(_) => {
            panic!(
                "Check llvm-dis version: command not found: {}",
                tool::LLVM_DIS
            );
        }
    }
}

/// Check settings of the LLVM disassembler (llvm-dis)
pub fn check_llvm_disassembler_settings() {
    check_llvm_disassembler_path();
    check_llvm_disassembler_version()
}

/// Disassemble an LLVM bitcode file
pub fn disassemble(input_file: &str) {
    // Check the tool settings
    check_llvm_disassembler_settings();

    // Start to disassemble the input file
    let input_file_path = Path::new(input_file);
    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));

    // prepare output file
    let output_file_path = parent_dir.join(file_stem_name.to_owned() + ".ll");
    let output_file_name = output_file_path.to_str().unwrap().to_string();
    fs::remove_file(output_file_name.as_str()).unwrap_or(());

    let llvm_dis_args =
        input_file.to_owned() + format!(" -o {}", output_file_name).as_str();

    // debug!("Running command: {} {}", tool::LLVM_DIS, llvm_dis_args);

    let llvm_dis_output = Command::new(tool::LLVM_DIS)
        .args(llvm_dis_args.split_whitespace())
        .output()
        .unwrap();

    if !llvm_dis_output.status.success() {
        let error_msg = String::from_utf8(llvm_dis_output.stderr.to_vec())
            .expect("llvm-dis: unknown error!");
        report::print_message("llvm-dis error message:", &error_msg);
        panic!("Failed to disassemble file: {}", input_file);
    }

    // debug!("Disassembled bitcode file to: {}", output_file_name)
}
