use std::error::Error;
use std::net::IpAddr;
use std::fs::File;
use std::io::Write;

fn main() {
    //
    //RUST_BACKTRACE=full cargo run --bin fifth_lesson
    //panic!("crash and burn");

    let home: IpAddr = "127.0.0.1".parse().unwrap();
    // let home: IpAddr = "127.0.0.1 hello".parse().expect("wanna be a Ipv4Addr ");
    // let home: IpAddr = "127.0.0.1 hello".parse().unwrap();
    println!("IpAddr {}", home);
    println!("IpAddr {}", home);

    // enum Result<T, E> {
    //      Ok(T),
    //      Err(E),
    //  }
    let _ = test_write_fn().unwrap();
    let _ = re_write_fn().unwrap();
    //Result 用于返回结果处理，? 用于错误的传播

    //? 用于 Option 的返回
    // pub enum Option<T> {
    //     Some(T),
    //     None
    // }
    let a = test_get_num();

}


fn test_write_fn() -> Result<usize, std::io::Error> {
    let f = File::create("./hello1.txt", );
    let mut f = match f {
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };
    let size = f.write("hello fifth_lesson".as_bytes());
    let size = match size {
        Ok(size) => size,
        Err(error) => {
            return Err(error);
        }
    };
    f.flush()?;
    Ok(size)
}


fn re_write_fn() -> Result<usize, Box<dyn Error>> {
    let mut f = File::create("./hello2.txt")?;
    let size = f.write("hello fifth_lesson".as_bytes())?;
    f.flush()?;
    Ok(size)
}


fn get_num(n: u32) -> Option<u32>{
    if n % 2 == 0{
        return Some(2)
    }
    None
}

fn test_get_num() -> Option<i32>{
    let nums = vec![1,2,3,4,5,6,7,8,8];
    let n = nums.first()?;
    Some(*n)
}