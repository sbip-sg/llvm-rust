//! Module invoking the LLVM optimization tool for bitcode (*.bc) files.

use regex::Regex;
use semver::{Version, VersionReq};
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::tool;
use rutil::{report, system};

use super::LLVM_REQUIRED_VERSION;

/// Check path of the LLVM optimization tool (llvm-opt)
fn check_llvm_optimization_path() {
    match system::path_of_command_from_env(tool::LLVM_OPT) {
        Ok(path) => debug!("llvm-opt path: {}", path),
        Err(_) => panic!("llvm-opt path not found: {}!", tool::LLVM_OPT),
    }
}

/// Check version of the LLVM optimization tool (llvm-opt)
fn check_llvm_optimization_version() {
    let llvm_opt_output =
        Command::new(tool::LLVM_OPT).args(&["--version"]).output();
    match llvm_opt_output {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"version (\d+\.\d+\.\d+)").unwrap();
            let llvm_opt_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            let llvm_opt_ver = match Version::parse(llvm_opt_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("Opt version not found: {}", msg),
            };
            let llvm_ver = match VersionReq::parse(LLVM_REQUIRED_VERSION) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("LLVM required version invalid: {}", msg)
                }
            };
            if !llvm_ver.matches(&llvm_opt_ver) {
                panic!(
                    "Expect Opt version {} but found: {}",
                    llvm_ver, llvm_opt_ver
                );
            }
        }

        Err(_) => {
            panic!("Check Opt version: command not found: {}", tool::LLVM_OPT);
        }
    }
}

/// Check settings of the LLVM optimization tool (llvm-opt)
pub fn check_llvm_optimization_settings() {
    check_llvm_optimization_path();
    check_llvm_optimization_version()
}

/// Optimize an LLVM bitcode file and return the output bitcode file name.
pub fn optimize(input_file: &str) -> String {
    // Check the tool settings
    check_llvm_optimization_settings();

    // Start to optimize file
    let input_file_path = Path::new(input_file);
    let file_stem_name = input_file_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));

    // prepare output file
    let out_file_path = parent_dir.join(file_stem_name.to_owned() + ".opt.bc");
    let out_file_name = out_file_path.to_str().unwrap();
    fs::remove_file(out_file_name).unwrap_or(());

    let llvm_opt_args = "--mem2reg".to_owned()
        + " --disable-verify"
        + format!(" {}", input_file).as_str()
        + format!(" -o {}", out_file_name).as_str();

    // debug!("Running command: {} {}", tool::LLVM_OPT, llvm_opt_args);

    let llvm_opt_output = Command::new(tool::LLVM_OPT)
        .args(llvm_opt_args.split_whitespace())
        .output()
        .unwrap();

    if !llvm_opt_output.status.success() {
        let error_msg = String::from_utf8(llvm_opt_output.stderr.to_vec())
            .expect("llvm-opt: unknown error!");
        report::print_message("llvm-opt error message:", error_msg.as_str());
        panic!("Llvm-opt: failed to optimize: {}", input_file);
    }

    out_file_name.to_string()
}
