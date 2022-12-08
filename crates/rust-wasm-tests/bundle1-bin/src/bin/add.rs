#![no_std]
#![no_main]

c2zk_stdlib::entry!(main);

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn main() {
    c2zk_rust_wasm_tests_bundle1::add::main();
}
