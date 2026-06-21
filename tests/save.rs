//! Test the --save argument

use assert_cmd::Command;
use std::fs;

#[test]
fn save_arg() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.args(["--save", "ls"]).assert().success();

    let output = "man_ls.pdf";
    let metadata = fs::metadata(output).unwrap();

    assert!(metadata.is_file());
    assert!(metadata.len() > 0);
}
