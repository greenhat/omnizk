#![no_std]
#![no_main]

c2zk_stdlib::entry!(main);

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn main() {
    c2zk_rust_wasm_tests_fib::fib::fib_seq();
}
