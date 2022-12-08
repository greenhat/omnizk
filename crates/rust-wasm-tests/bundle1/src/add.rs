use c2zk_stdlib::*;

#[inline(never)]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

pub fn main() {
    let a = pub_input();
    let b = pub_input();
    let r = add(a, b);
    let c = secret_input();
    let r2 = add(r, c);
    pub_output(r2);
}
