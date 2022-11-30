extern "C" {
    fn c2zk_stdlib_pub_input() -> u64;
    fn c2zk_stdlib_pub_output(x: u64);
}

pub fn pub_input() -> u64 {
    unsafe { c2zk_stdlib_pub_input() }
}

pub fn pub_output(x: u64) {
    unsafe { c2zk_stdlib_pub_output(x) }
}
