use std::{
    fs::{self, File},
    io::Write,
};

use rust_wasm_github_actions_test::get_json_cst_core;

#[test]
fn write_cst() {
    let json_str = fs::read_to_string("testfiles/src.json").unwrap();
    let cst = get_json_cst_core(&json_str);
    let mut out_file = File::create("testfiles/dst.txt").unwrap();
    out_file.write_all(cst.as_bytes()).unwrap();
}
