#![no_std]
#![no_main]

c2zk_stdlib::entry!(main_add_bin);

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn main_add_bin() {
    c2zk_rust_wasm_tests_add::add::main_add();
}
