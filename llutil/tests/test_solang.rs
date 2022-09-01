#[cfg(test)]
use llutil::tool::solang;
use serial_test::serial;

#[test]
#[serial]
fn test_solang_compile() {
    let input_file = "tests/testcases/solidity/simple_storage.sol";
    let options = vec![" -g"];
    let output_files = solang::compile(input_file, &options);

    // Only 1 output file for this contract
    assert_eq!(output_files.len(), 1);

    // The output file is a bitcode file
    assert!(output_files.iter().all(|fname| fname.ends_with(".bc")));
}
