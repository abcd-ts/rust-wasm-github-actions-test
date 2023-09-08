#[no_mangle]
pub extern "C" fn increment(n: u32) -> u32 {
    n + 1
}

#[allow(dead_code)]
fn sum(array: &Vec<i32>) -> i32 {
    array.iter().sum()
}

#[cfg(test)]
#[test]
fn increment_test() {
    assert_eq!(increment(5), 5 + 1)
}
