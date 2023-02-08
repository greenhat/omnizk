use c2zk_stdlib::*;

pub fn fib_seq() {
    let n = pub_input();
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    pub_output(a);
}
