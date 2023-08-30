#[no_mangle]
pub extern "C" fn increment(n: u32) -> u32 {
    n + 1
}
