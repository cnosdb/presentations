use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::thread::{spawn,sleep};
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("started changed");
}

// fn main() {
//     let flag = Arc::new(Mutex::new(false));
//     let cond = Arc::new(Condvar::new());
//     let cflag = flag.clone();
//     let ccond = cond.clone();
//
//     let hdl = spawn(move || {
//         let mut m = { *cflag.lock().unwrap() };
//         let mut counter = 0;
//
//         while counter < 3 {
//             while !m {
//                 m = *ccond.wait(cflag.lock().unwrap()).unwrap();
//             }
//
//             {
//                 m = false;
//                 *cflag.lock().unwrap() = false;
//             }
//
//             counter += 1;
//             println!("inner counter: {}", counter);
//         }
//     });
//
//     let mut counter = 0;
//     loop {
//         sleep(Duration::from_millis(1000));
//         *flag.lock().unwrap() = true;
//         counter += 1;
//         if counter > 3 {
//             break;
//         }
//         println!("outside counter: {}", counter);
//         cond.notify_one();
//     }
//     hdl.join().unwrap();
//     println!("{:?}", flag);
// }
