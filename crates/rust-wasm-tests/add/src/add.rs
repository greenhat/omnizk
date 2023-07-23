use ozk_stdlib::*;

#[inline(never)]
#[no_mangle]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[no_mangle]
pub fn main_add() {
    let a = pub_input();
    let b = pub_input();
    let r = add(a, b);
    let c = secret_input();
    let r2 = add(r, c);
    pub_output(r2);
}
