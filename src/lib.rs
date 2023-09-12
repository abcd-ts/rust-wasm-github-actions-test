pub fn increment(n: u32) -> u32 {
    n + 1
}

pub fn get_json_cst_core(json_str: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_json::language()).unwrap();
    let tree = parser.parse(json_str, None).unwrap();

    format!("{tree:?}")
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
