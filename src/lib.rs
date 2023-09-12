use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub extern "C" fn increment(n: u32) -> u32 {
    n + 1
}

#[no_mangle]
pub unsafe extern "C" fn get_json_cst(json_str: *mut c_char) -> *mut c_char {
    let json_str = CStr::from_ptr(json_str).to_str().unwrap().to_owned();

    CString::new(get_json_cst_core(&json_str))
        .unwrap()
        .into_raw()
}

fn get_json_cst_core(json_str: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_json::language()).unwrap();
    let tree = parser.parse(json_str, None).unwrap();

    format!("{:?}", tree)
}

#[no_mangle]
pub unsafe extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = CString::from_raw(ptr);
    };
}

#[allow(dead_code)]
fn sum(array: &[i32]) -> i32 {
    array.iter().sum()
}

#[cfg(test)]
#[test]
fn increment_test() {
    assert_eq!(increment(5), 5 + 1)
}

#[cfg(test)]
#[test]
fn get_json_cst_test() {
    let src = r#"{hoge: "fuga"}"#;
    assert_eq!(
        get_json_cst_core(src),
        "{Tree {Node document (0, 0) - (0, 14)}}".to_owned()
    )
}
