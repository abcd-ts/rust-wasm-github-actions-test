#![deny(clippy::all)]

use rust_wasm_github_actions_test::get_json_cst_core;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn get_json_cst(json_str: String) -> String {
  get_json_cst_core(&json_str)
}
