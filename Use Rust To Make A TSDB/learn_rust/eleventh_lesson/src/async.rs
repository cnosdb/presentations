use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world();
    block_on(future);
}


// async fn hello_world() {
//     hello_cat();
//     println!("hello, world!");
// }
//
// async fn hello_cat() {
//     println!("hello, cat!");
// }
// fn main() {
//     let future = hello_world();
//     block_on(future);
// }


// struct Song {
//     author: String,
//     name: String,
// }
//
// async fn learn_song() -> Song {
//     Song {
//         author: "bird".to_string(),
//         name: String::from("ji ji zha zha"),
//     }
// }
//
// async fn sing_song(song: Song) {
//     println!(
//         "I like a {}: {} {}",
//         song.author, song.name, "ji ji zha zha"
//     );
// }
//
// async fn dance() {
//     println!("dancing");
// }

// fn main() {
//     let song = block_on(learn_song());
//     block_on(sing_song(song));
//     block_on(dance());
// }
//
// async fn learn_and_sing() {
//     let song = learn_song().await;
//
//     sing_song(song).await;
// }
//
//
// async fn async_main() {
//     let f1 = learn_and_sing();
//     let f2 = dance();
//
//     futures::join!(f1, f2);
// }
//
// fn main() {
//     block_on(async_main());
// }

