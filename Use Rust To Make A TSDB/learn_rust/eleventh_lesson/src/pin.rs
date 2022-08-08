#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        unsafe { &*(self.b) }
    }
}

fn main() {
    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();

    println!("a: {}, b: {}", test1.a(), test1.b());
    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b: {}", test2.a(), test2.b());
}





// use std::pin::Pin;
// use std::marker::PhantomPinned;
//
// #[derive(Debug)]
// struct Test {
//     a: String,
//     b: *const String,
//     _marker: PhantomPinned,
// }
//
//
// impl Test {
//     fn new(txt: &str) -> Self {
//         Test {
//             a: String::from(txt),
//             b: std::ptr::null(),
//             _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
//         }
//     }
//
//     fn init(self: Pin<&mut Self>) {
//         let self_ptr: *const String = &self.a;
//         let this = unsafe { self.get_unchecked_mut() };
//         this.b = self_ptr;
//     }
//
//     fn a(self: Pin<&Self>) -> &str {
//         &self.get_ref().a
//     }
//
//     fn b(self: Pin<&Self>) -> &String {
//         unsafe { &*(self.b) }
//     }
// }
//
//
// pub fn main() {
//     let mut test1 = Test::new("test1");
//     let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
//     Test::init(test1.as_mut());
//
//     let mut test2 = Test::new("test2");
//     let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
//     Test::init(test2.as_mut());
//
//     println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
//     std::mem::swap(test1.get_mut(), test2.get_mut());
//     println!("a: {}, b: {}", Test::a(test2.as_ref()), Test::b(test2.as_ref()));
// }

