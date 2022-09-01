//! Module invoking the Solang compiler tool for Solidity files.

use regex::Regex;
use semver::{Version, VersionReq};
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::file::ext;
use crate::tool::{self, OUTPUT_DIR};
use rutil::string::StringUtil;
use rutil::{report, system};

/// Required Solang version
const SOLANG_REQUIRED_VERSION: &str = ">=0.1.13";

/// Check path of the Solang compiler
fn check_solang_path() {
    match system::path_of_command_from_env(tool::SOLANG) {
        Ok(path) => debug!("Solang path: {}", path),
        Err(_) => panic!("Solang path not found!"),
    }
}

/// Check version of the Solang compiler
pub fn check_solang_version() {
    match Command::new(tool::SOLANG).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"version v(\d+\.\d+\.\d+)").unwrap();
            let solang_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            let solang_ver = match Version::parse(solang_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("Solang version not found: {}", msg),
            };
            let solang_ver_req =
                match VersionReq::parse(SOLANG_REQUIRED_VERSION) {
                    Ok(ver) => ver,
                    Err(msg) => {
                        panic!("Solang required version invalid: {}", msg)
                    }
                };
            if !solang_ver_req.matches(&solang_ver) {
                panic!(
                    "Expect Solang version {} but found: {}",
                    solang_ver_req, solang_ver
                );
            }
        }

        Err(_) => {
            panic!("Check Solang version: command not found: {}", tool::SOLANG);
        }
    }
}

/// Check settings of the Solang compiler
pub fn check_solang_settings() {
    check_solang_path();
    check_solang_version()
}

/// Compile Solidity programs and return the output bitcode file name.
pub fn compile(input_file: &str, user_options: &[&str]) -> Vec<String> {
    // Check compiler settings
    check_solang_settings();

    // Start to compile the input file
    let input_file_path = Path::new(input_file);
    let filename = input_file_path
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    let parent_dir = input_file_path.parent().unwrap_or_else(|| Path::new(""));

    // prepare output folder
    let output_dir = parent_dir.join(OUTPUT_DIR).join(filename);
    let output_dir_path = output_dir.to_str().unwrap();
    fs::remove_dir_all(output_dir_path).unwrap_or(());
    fs::create_dir_all(output_dir_path).unwrap_or(());

    let solang_args = "compile ".to_owned()
        + input_file
        + &user_options.join(" ").add_prefix_if_not_empty(" ")
        + " -O none"
        + " --no-constant-folding"
        + " --no-strength-reduce"
        + " --no-dead-storage"
        + " --no-vector-to-slice"
        + " --target solana"
        + " --emit llvm-bc"
        + format!(" -o {}", output_dir_path).as_str();

    // debug!("Running command: {} {}", tool::SOLANG, solang_args);

    let solang_output = Command::new(tool::SOLANG)
        .args(solang_args.split_whitespace())
        .output()
        .unwrap();

    if !solang_output.status.success() {
        let error_msg = String::from_utf8(solang_output.stderr.to_vec())
            .expect("Solang: unknown error!");
        report::print_message("Solang error message:", error_msg.as_str());
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
