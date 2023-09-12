use std::ffi::{c_char, CStr, CString};

use rust_wasm_github_actions_test::get_json_cst_core;

#[no_mangle]
pub extern "C" fn increment(n: u32) -> u32 {
    n + 1
}

/// # Safety
/// dummy
#[no_mangle]
pub unsafe extern "C" fn get_json_cst(json_str: *mut c_char) -> *mut c_char {
    let json_str = CStr::from_ptr(json_str).to_str().unwrap().to_owned();

    CString::new(get_json_cst_core(&json_str))
        .unwrap()
        .into_raw()
}

/// # Safety
/// dummy
#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    };
}
