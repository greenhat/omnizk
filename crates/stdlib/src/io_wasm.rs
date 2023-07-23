extern "C" {
    fn ozk_stdlib_pub_input() -> u64;
    fn ozk_stdlib_pub_output(x: u64);
    fn ozk_stdlib_secret_input() -> u64;
}

pub fn pub_input() -> u64 {
    unsafe { ozk_stdlib_pub_input() }
}

pub fn pub_output(x: u64) {
    unsafe { ozk_stdlib_pub_output(x) }
}

pub fn secret_input() -> u64 {
    unsafe { ozk_stdlib_secret_input() }
}
