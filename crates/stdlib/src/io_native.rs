use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref PUB_INPUT: Mutex<Vec<u64>> = Mutex::new(vec![]);
    static ref PUB_OUTPUT: Mutex<Vec<u64>> = Mutex::new(vec![]);
}

pub fn set_pub_input(input: Vec<u64>) {
    *PUB_INPUT.lock() = input;
}

pub fn get_pub_output() -> Vec<u64> {
    PUB_OUTPUT.lock().clone()
}

pub fn pub_input() -> u64 {
    #[allow(clippy::unwrap_used)]
    PUB_INPUT.lock().pop().unwrap()
}

pub fn pub_output(x: u64) {
    PUB_OUTPUT.lock().push(x);
}
