/// compile folder for solana

use regex::Regex;
use semver::{Version, VersionReq};
use std::path;
use std::{ffi::OsStr, fs, path::Path, process::Command};

use crate::file::ext;
use crate::tool::{self, OUTPUT_DIR};
use rutil::string::StringExt;
use rutil::{report, system};


/// Required cargo version
/// command : cargo build-bpf
// TODO: About the cargo version
const CARGO_VERSION_REQ: &str = ">=0.8.11";
const CARGO_BUILD_SBF_VERSION_REQ: &str = ">=1.0.0";

/// check path of cargo
fn check_cargo_path() {
    match system::path_of_command_from_env(tool::CARGO) {
        Ok(path) => debug!("Cargo path: {}", path),
        Err(_) => panic!("Cargo path not found!"),
    }
}

fn check_cargo_build_sbf_path() {
    match system::path_of_command_from_env(tool::CARGO_BUILD_SBF) {
        Ok(path) => debug!("Cargo-build-sbf path: {}", path),
        Err(_) => panic!("Cargo-build-sbf not found!"),
    }
}


/// Check version of the cargo
pub fn check_cargo_version() {
    match Command::new(tool::CARGO).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"cargo (\d+\.\d+\.\d+)\s\(\w+\s\d+-\d+-\d+\)").unwrap();
            dbg!(output_str.as_str());
            let cargo_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };
            dbg!(cargo_ver);
            let cargo_ver = match Version::parse(cargo_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("Cargo version not found: {}", msg),
            };
            let ver_required = match VersionReq::parse(CARGO_VERSION_REQ) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("Cargo required version invalid: {}", msg)
                }
            };
            if !ver_required.matches(&cargo_ver) {
                panic!(
                    "Expect Cargo version {} but found: {}",
                    ver_required, cargo_ver
                );
            }
        }

        Err(_) => {
            panic!("Check Cargo version: command not found: {}", tool::CARGO);
        }
    }
}

/// Check version of the cargo-build-sbf
pub fn check_cargo_build_sbf_version() {
    match Command::new(tool::CARGO_BUILD_SBF).args(&["--version"]).output() {
        Ok(output) => {
            let output_str = String::from_utf8(output.stdout).unwrap();
            let regex = Regex::new(r"solana-cargo-build-sbf (\d+\.\d+\.\d+)").unwrap();
            dbg!(output_str.as_str());
            let cargo_build_sbf_ver = match regex.captures(output_str.as_str()) {
                Some(capture) => capture.get(1).map_or("", |c| c.as_str()),
                None => "",
            };

            dbg!(cargo_build_sbf_ver);
            let cargo_build_sbf_ver = match Version::parse(cargo_build_sbf_ver) {
                Ok(ver) => ver,
                Err(msg) => panic!("cargo-build-sbf version not found: {}", msg),
            };
            let ver_required = match VersionReq::parse(CARGO_BUILD_SBF_VERSION_REQ) {
                Ok(ver) => ver,
                Err(msg) => {
                    panic!("cargo-build-sbf required version invalid: {}", msg)
                }
            };
            if !ver_required.matches(&cargo_build_sbf_ver) {
                panic!(
                    "Expect cargo-build-sbf version {} but found: {}",
                    ver_required, cargo_build_sbf_ver
                );
            }
        }

        Err(_) => {
            panic!("Check cargo-build-sbf version: command not found: {}", tool::CARGO_BUILD_SBF);
        }
    }
}

/// Check settings of the cargo
pub fn check_cargo_settings() {
    check_cargo_path();
    check_cargo_version()
}

pub fn check_cargo_build_sbf_settings() {
    check_cargo_build_sbf_path();
    check_cargo_build_sbf_version()
}

/// Compile Solana programs and return the output file path.
pub fn compile(input_file: &str, user_options: &[&str]) -> Vec<String> {
    // Check compiler settings
    check_cargo_settings();
    check_cargo_build_sbf_settings();

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

    let toml_path = match input_file_path.to_str(){
        Some(path) => path.to_owned() + "/Cargo.toml",
        None => "".to_owned()
    };

    let user_options = user_options.join(" ");
    let solana_args = user_options.add_prefix_if_not_empty(" ")
                            + "--manifest-path " + &toml_path;

    debug!("Running command: {} {}", tool::CARGO_BUILD_BPF, solana_args);

    let solana_output = Command::new(tool::CARGO_BUILD_BPF)
        .args(solana_args.split_whitespace())
        .output()
        .unwrap();

    if !solana_output.status.success() {
        let error_msg = String::from_utf8(solana_output.stderr.to_vec())
            .expect("Solana: unknown error!");
        report::print_message("Solana error message:", error_msg.as_str());
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
