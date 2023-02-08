use std::cell::RefCell;
use std::thread_local;
use std::vec::Vec;

thread_local! {
    static PUB_INPUT: RefCell<Vec<u64>> = RefCell::new(vec![]);
    static PUB_OUTPUT: RefCell<Vec<u64>> = RefCell::new(vec![]);
    static SECRET_INPUT: RefCell<Vec<u64>> = RefCell::new(vec![]);
}

pub fn init_io(pub_input: Vec<u64>, secret_input: Vec<u64>) {
    let mut pub_input_reversed = pub_input;
    pub_input_reversed.reverse();
    let mut secret_input_reversed = secret_input;
    secret_input_reversed.reverse();
    PUB_INPUT.with(|v| {
        *v.borrow_mut() = pub_input_reversed;
    });
    SECRET_INPUT.with(|v| {
        *v.borrow_mut() = secret_input_reversed;
    });
    PUB_OUTPUT.with(|v| {
        *v.borrow_mut() = vec![];
    });
}

pub fn get_pub_output() -> Vec<u64> {
    PUB_OUTPUT.with(|v| v.borrow().clone())
}

pub(crate) fn pub_input() -> u64 {
    #[allow(clippy::unwrap_used)]
    PUB_INPUT.with(|v| v.borrow_mut().pop().unwrap())
}

pub(crate) fn pub_output(x: u64) {
    PUB_OUTPUT.with(|v| v.borrow_mut().push(x));
}

pub(crate) fn secret_input() -> u64 {
    #[allow(clippy::unwrap_used)]
    SECRET_INPUT.with(|v| v.borrow_mut().pop().unwrap())
}
