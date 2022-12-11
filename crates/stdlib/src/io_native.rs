use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref PUB_INPUT: Mutex<Vec<u64>> = Mutex::new(vec![]);
    static ref PUB_OUTPUT: Mutex<Vec<u64>> = Mutex::new(vec![]);
    static ref SECRET_INPUT: Mutex<Vec<u64>> = Mutex::new(vec![]);
}

pub fn init_io(pub_input: Vec<u64>, secret_input: Vec<u64>) {
    //TODO: do not share this state with other runs (parallel test runs)
    let mut pub_input_reversed = pub_input;
    pub_input_reversed.reverse();
    let mut secret_input_reversed = secret_input;
    secret_input_reversed.reverse();
    *PUB_INPUT.lock() = pub_input_reversed;
    *SECRET_INPUT.lock() = secret_input_reversed;
    *PUB_OUTPUT.lock() = vec![];
}

pub fn get_pub_output() -> Vec<u64> {
    PUB_OUTPUT.lock().clone()
}

pub(crate) fn pub_input() -> u64 {
    #[allow(clippy::unwrap_used)]
    PUB_INPUT.lock().pop().unwrap()
}

pub(crate) fn pub_output(x: u64) {
    PUB_OUTPUT.lock().push(x);
}

pub(crate) fn secret_input() -> u64 {
    #[allow(clippy::unwrap_used)]
    SECRET_INPUT.lock().pop().unwrap()
}
