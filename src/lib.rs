#[no_mangle]
pub extern "C" fn increment(n: u32) -> u32 {
    n + 1
}

#[cfg(test)]
#[test]
fn increment_test() {
  assert_eq!(increment(5), 5 + 1)
}
