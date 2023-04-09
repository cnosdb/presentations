mod pattern;
use crate::pattern::singleton::{do_a_call, ARRAY};

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("Called {} times", ARRAY.lock().unwrap().len());
}
