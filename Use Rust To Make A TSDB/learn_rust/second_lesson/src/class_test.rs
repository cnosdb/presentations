//first question
fn q1() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

//fix
fn q2() {
    assert!(0.1 + 0.2 == 0.3);
}

fn q3() {
    let s = "hello, world".to_string();
    let s1: str = &s;
}

fn q4() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
    match numbers {
        _ => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
}
fn main() {

}
