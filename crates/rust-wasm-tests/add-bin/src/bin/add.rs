#![no_std]
#![no_main]

ozk_stdlib::entry!(main_add_bin);

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn main_add_bin() {
    ozk_rust_wasm_tests_add::add::main_add();
}
