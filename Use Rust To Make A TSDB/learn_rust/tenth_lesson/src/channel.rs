use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // m multi p producer s single c consumer
    let (tx, rx) = mpsc::channel();


    thread::spawn(move || {
        tx.send(1).unwrap();

        // tx.send(Some(1)).unwrap()
    });

    println!("receive {}", rx.recv().unwrap());
}


// 使用for循环接收
// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// 多发送者
// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         tx.send(String::from("hi from raw tx")).unwrap();
//     });
//
//     thread::spawn(move || {
//         tx1.send(String::from("hi from cloned tx")).unwrap();
//     });
//
//     for received in rx {
//         println!("Got: {}", received);
//     }
// }


// 同步channel
// fn main() {
//     let (tx, rx)= mpsc::sync_channel(0);
//
//     let handle = thread::spawn(move || {
//         println!("发送之前");
//         tx.send(1).unwrap();
//         println!("发送之后");
//     });
//
//     println!("睡眠之前");
//     thread::sleep(Duration::from_secs(3));
//     println!("睡眠之后");
//
//     println!("receive {}", rx.recv().unwrap());
//     handle.join().unwrap();
// }
