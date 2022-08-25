//! Module invoking the Clang compiler for C/C++ files.

use regex::Regex;
use semver::{Version, VersionReq};
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::tool::{self, OUTPUT_DIR};
use rutil::string::StringUtil;
use rutil::{report, system};

use super::LLVM_REQUIRED_VERSION;

/// Checking path of Clang
fn check_clang_path() {
    match system::path_of_command_from_env(tool::CLANG) {
        Ok(path) => debug!("Clang path: {}", path),
        Err(_) => panic!("Clang path not found!"),
    }
}

/// Checking version of Clang
fn check_clang_version() {
    match Command::new(tool::CLANG).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"version (\d+\.\d+\.\d+)").unwrap();
            let clang_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            let clang_ver = match Version::parse(clang_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("Clang version not found: {}", msg),
            };
            let llvm_ver = match VersionReq::parse(LLVM_REQUIRED_VERSION) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("Clang required version invalid: {}", msg)
                }
            };
            if !llvm_ver.matches(&clang_ver) {
                panic!(
                    "Expect Clang version {} but found: {}",
                    llvm_ver, clang_ver
                );
            }
        }

        Err(_) => {
            panic!("Check Clang version: command not found: {}", tool::CLANG);
        }
    }
}

/// Checking path of Clang
pub fn check_clang_settings() {
    check_clang_path();
    check_clang_version()
}

/// Compile C/C++ programs and return the output bitcode file name.
pub fn compile(
    input_file: &str,
    user_options: &[&str],
    include_dirs: &[&str],
    include_files: &[&str],
) -> Vec<String> {
    // Check compiler settings
    check_clang_settings();

    // Start to compile the input file
    let input_file_path = Path::new(input_file);
    let input_file_stem = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));

    // Prepare output folder
    let output_dir = parent_dir.join(OUTPUT_DIR).join(input_file_stem);
    let output_dir_name = output_dir.to_str().unwrap();
    fs::remove_dir(output_dir_name).unwrap_or(());
    fs::create_dir_all(output_dir_name).unwrap_or(());

    // Compile source code files
    let mut user_options = user_options.join(" ");
    for dir in include_dirs {
        user_options = user_options.to_owned() + " -I " + dir;
    }
    let clang_args = user_options.add_prefix_if_not_empty(" ")
        + " -g -O0 -fno-rtti"
        + " -Xclang -disable-llvm-passes"
        + " -Xclang -disable-O0-optnone"
        + " -Werror=implicit-function-declaration"
        + " -c -emit-llvm";
    let source_files = [&[input_file], include_files].concat();
    let mut output_files = Vec::new();
    for file in source_files {
        let file_stem = Path::new(file)
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or("");
        let output_file = output_dir.join(file_stem.to_owned() + ".bc");
        let clang_args = file.to_owned()
            + &clang_args
            + &format!(" -o {}", output_file.to_str().unwrap());

        // debug!("Running command: {} {}", tool::CLANG, clang_args);

        let clang_output = Command::new(tool::CLANG)
            .args(clang_args.split_whitespace())
            .output()
            .unwrap();

        if !clang_output.status.success() {
            let error_msg = String::from_utf8(clang_output.stderr.to_vec())
                .expect("clang: unknown error!");
            report::print_message("Clang error message:", error_msg.as_str());
            panic!("Failed to compile: {}", input_file);
        }

        output_files.push(output_file.to_str().unwrap().to_owned());
    }

    // Combine to final output file.
    if include_files.is_empty() {}
    let final_output_path =
        output_dir.join(input_file_stem.to_owned() + ".raw.bc");
    let final_output_file = final_output_path.to_str().unwrap();

    let llvm_link_args =
        output_files.join(" ") + &format!(" -o {}", final_output_file);

    // debug!("Running command: {} {}", tool::LLVM_LINK, llvm_link_args);

    let llvm_link_output = Command::new(tool::LLVM_LINK)
        .args(llvm_link_args.split_whitespace())
        .output()
        .unwrap();

    if !llvm_link_output.status.success() {
        let error_msg = String::from_utf8(llvm_link_output.stderr.to_vec())
            .expect("clang: unknown error!");
        report::print_message("Clang error message:", error_msg.as_str());
        panic!("Failed to compile: {}", input_file);
    }

    vec![final_output_file.to_owned()]
}
