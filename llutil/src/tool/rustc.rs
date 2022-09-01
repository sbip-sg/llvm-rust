//! Module invoking the Solc compiler tool for Solidity files.

use regex::Regex;
use semver::{Version, VersionReq};
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::file::ext;
use crate::tool::{self, OUTPUT_DIR};
use rutil::string::StringExt;
use rutil::{report, system};

/// Required Solc version
const RUSTC_VERSION_REQ: &str = ">=0.8.11";

/// Check path of the Rustc compiler
fn check_rustc_path() {
    match system::path_of_command_from_env(tool::RUSTC) {
        Ok(path) => debug!("Rustc path: {}", path),
        Err(_) => panic!("Rustc path not found!"),
    }
}

/// Check version of the Rustc compiler
pub fn check_rustc_version() {
    match Command::new(tool::RUSTC).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"Version: (\d+\.\d+\.\d+)").unwrap();
            let rustc_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            let rustc_ver = match Version::parse(rustc_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("Rustc version not found: {}", msg),
            };
            let ver_required = match VersionReq::parse(RUSTC_VERSION_REQ) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("Rustc required version invalid: {}", msg)
                }
            };
            if !ver_required.matches(&rustc_ver) {
                panic!(
                    "Expect Rustc version {} but found: {}",
                    ver_required, rustc_ver
                );
            }
        }

        Err(_) => {
            panic!("Check Rustc version: command not found: {}", tool::RUSTC);
        }
    }
}

/// Check settings of the Rustc compiler
pub fn check_rustc_settings() {
    check_rustc_path();
    check_rustc_version()
}

/// Compile Solidity programs and return the output bitcode file name.
pub fn compile(input_file: &str, user_options: &[&str]) -> Vec<String> {
    // Check compiler settings
    check_rustc_settings();

    // Start to compile the input file
    let input_file_path = Path::new(&input_file);
    let filename = input_file_path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));

    // Prepare output folder
    let output_dir = parent_dir.join(OUTPUT_DIR).join(filename);
    let output_dir_path = output_dir.to_str().unwrap();
    fs::remove_dir_all(output_dir_path).unwrap_or(());
    fs::create_dir_all(output_dir_path).unwrap_or(());

    let rustc_args = input_file.to_owned()
        + &user_options.join(" ").add_prefix_if_not_empty(" ")
        + " --emit llvm-bc"
        + format!(" -o {}", output_dir_path).as_str();

    // debug!("Running command: {} {}", tool::RUSTC, rustc_args);

    let rustc_output = Command::new(tool::RUSTC)
        .args(rustc_args.split_whitespace())
        .output()
        .unwrap();

    if !rustc_output.status.success() {
        let error_msg = String::from_utf8(rustc_output.stderr.to_vec())
            .expect("Rustc: unknown error!");
        report::print_message("Rustc error message:", error_msg.as_str());
        panic!("Failed to compile: {}", input_file);
    }

    system::ls_dir(output_dir_path)
        .into_iter()
        .filter_map(|filename: String| -> Option<String> {
            if filename.ends_with(ext::BC) {
                Some(filename)
            } else {
                None
            }
        })
        .collect()
}
