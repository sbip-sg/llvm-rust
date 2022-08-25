//! Module invoking the Solc compiler tool for Solidity files.

use regex::Regex;
use semver::{Version, VersionReq};
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::file::ext;
use crate::tool::{self, OUTPUT_DIR};
use rutil::string::StringUtil;
use rutil::{report, system};

/// Required Solc version
pub const SOLC_REQUIRED_VERSION: &str = ">=0.8.11";

/// Check path of the Solc compiler
fn check_solc_path() {
    match system::path_of_command_from_env(tool::SOLC) {
        Ok(path) => debug!("Solc path: {}", path),
        Err(_) => panic!("Solc path not found!"),
    }
}

/// Check version of the Solc compiler
pub fn check_solc_version() {
    match Command::new(tool::SOLC).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"Version: (\d+\.\d+\.\d+)").unwrap();
            let solc_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            let solc_ver = match Version::parse(solc_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("Solc version not found: {}", msg),
            };
            let ver_required = match VersionReq::parse(SOLC_REQUIRED_VERSION) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("Solc required version invalid: {}", msg)
                }
            };
            if !ver_required.matches(&solc_ver) {
                panic!(
                    "Expect Solc version {} but found: {}",
                    ver_required, solc_ver
                );
            }
        }

        Err(_) => {
            panic!("Check Solc version: command not found: {}", tool::SOLC);
        }
    }
}

/// Check settings of the Solc compiler
pub fn check_solc_settings() {
    check_solc_path();
    check_solc_version()
}

/// Compile Solidity smart contracts into YUL IRs.
/// It is a core file, other functions just need to add more options and call
/// this function.
///
/// Input: The source `input_file`. `options` contains all options to run
/// `input_file` using solc
pub fn compile(
    input_file: &str,
    options: &str,
    extension: &str,
) -> Vec<String> {
    // Check compiler settings
    check_solc_settings();

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

    for filename in system::ls_dir(output_dir_path) {
        if filename.ends_with(extension) {
            fs::remove_file(filename).unwrap();
        }
    }
    fs::create_dir_all(output_dir_path).unwrap_or(());

    let solc_args = input_file.to_owned()
        + options
        + format!(" -o {}", output_dir_path).as_str();

    // debug!("Running command: {} {}", tool::SOLC, solc_args);

    let solc_output = Command::new(tool::SOLC)
        .args(solc_args.split_whitespace())
        .output()
        .unwrap();

    if !solc_output.status.success() {
        let error_msg = String::from_utf8(solc_output.stderr.to_vec())
            .expect("Solc: unknown error!");
        report::print_message("Solc error message:", error_msg.as_str());
        panic!("Failed to compile: {}", input_file);
    }

    system::ls_dir(output_dir_path)
        .into_iter()
        .filter_map(|filename| -> Option<String> {
            if filename.ends_with(extension) {
                Some(filename)
            } else {
                None
            }
        })
        .collect()
}

/// Compile Solidity smart contract to EVM bytecode.
///
/// Input: a smart contract file name.
///
/// Output: a vector of EVM bytecode file name.
pub fn compile_to_evm_bytecode(
    file: &str,
    user_options: &[&str],
) -> Vec<String> {
    let options = user_options.join(" ").add_prefix_if_not_empty(" ");
    let options = options + " --asm";
    compile(file, &options, ext::EVM)
}

/// Compile Solidity smart contracts into AST in JSON format.
///
/// Input: A smart contract file name.
///
/// Output: a vector of output JSON file names which contain the ASTs.
///
pub fn compile_to_json_ast(file: &str, user_options: &[&str]) -> Vec<String> {
    let mut options = user_options.join(" ").add_prefix_if_not_empty(" ");
    options.push_str(" --asm-json");
    compile(file, &options, ext::EVM)
}

/// Compile Solidity smart contracts into YUL IRs.
///
/// Input: A smart contract file name.
///
/// Output: A YUL file containing intermediate representations in YUL.
///
pub fn compile_to_yul(file: &str, user_options: &[&str]) -> Vec<String> {
    let options = user_options.join(" ").add_prefix_if_not_empty(" ");
    let options = options + " --ir";
    compile(file, &options, ext::YUL)
}
