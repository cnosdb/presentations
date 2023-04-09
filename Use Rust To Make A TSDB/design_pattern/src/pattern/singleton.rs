use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
}

// pub static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new() );

pub fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

#[allow(dead_code)]
fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("Called {} times", ARRAY.lock().unwrap().len());
}
