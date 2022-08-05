use std::thread;
use std::rc::Rc;
use std::sync::Arc;
fn main() {
    let v = Rc::new(5);
    let t = thread::spawn(move || {
        println!("{}",v);
    });

    t.join().unwrap();
}


// #[derive(Debug)]
// struct SendStruct();
// unsafe impl Send for SendStruct {}
// fn main() {
//     let p = SendStruct();
//     let t = thread::spawn(move || {
//         println!("{:?}",p);
//     });
//
//     t.join().unwrap();
// }
