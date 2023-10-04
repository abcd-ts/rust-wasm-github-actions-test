// unuse
use std;

pub fn increment(n: u32) -> u32 {
    n + 1
}

pub fn get_json_cst_core(json_str: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_json::language()).unwrap();
    let tree = parser.parse(json_str, None).unwrap();

    format!("{tree:?}")
}

pub fn get_sql_cst_core(sql_str: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_sql::language()).unwrap();
    let tree = parser.parse(sql_str, None).unwrap();

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

#[cfg(test)]
#[test]
fn get_sql_cst_test() {
    let src = r#"select a from b"#;
    assert_eq!(
        get_sql_cst_core(src),
        "{Tree {Node source_file (0, 0) - (0, 15)}}".to_owned()
    )
}
