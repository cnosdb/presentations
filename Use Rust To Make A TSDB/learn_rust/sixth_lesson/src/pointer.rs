
//引用
// fn main() {
//     let a = [1,2,3];
//     let b = &a;
//     println!("{:p}", b); // 0x16b18f0b4
//
//     // 要获取可变引用，必须先声明可变绑定
//     let mut c = vec![1,2,3];
//     // 通过 &mut 得到可变引用
//     let d = &mut c;
//     d.push(4);
//     println!("{:?}", d); // [1, 2, 3, 4]
//
//     let e = &42;
//     assert_eq!(42, *e);
//
//     compare();
// }

// 引用与借用的关系
// 若B是对A的引用，也可称之为B借用了A。
// https://zhuanlan.zhihu.com/p/59998584
// 引用(references)是类型，类似于c/c++的指针
// 借用(borrowing)是运算，类似于c/c++的取地址
// 变量绑定中的&是模式匹配，效果就是解引用
// ref是关键字，作用是绑定对象的引用
fn compare(){

    let ref t;  t = &1;
    let m;  m = &1;
    let ref n:i32;  n = &1;

    let ref a=2;
    let ref b = &2;

    let r=&1;
    let &a=r;
    let a=*r;
}

//
fn main(){
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;

    // 原生指针操作要放在unsafe中执行
    unsafe {
        *ptr_x += *ptr_y;
    }
    // *ptr_x += *ptr_y;
    assert_eq!(x, 30);
}
