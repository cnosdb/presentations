
/*
    迭代器 Iterator
*/


fn main() {

    let values = vec![1, 2, 3];

    // for 循环通过不停调用迭代器上的 next 方法
    // for v in values.into_iter() {
    //     println!("{}", v)
    // }
    for v in values{
        println!("{}", v)
    }
    // impl<I: Iterator> IntoIterator for I {
    //     type Item = I::Item;
    //     type IntoIter = I;
    //
    //     #[inline]
    //     fn into_iter(self) -> I {
    //         self
    //     }
    // }
    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    // 迭代器与所有权
    // into_iter 会夺走所有权
    // iter 是借用
    // iter_mut 是可变借用
    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    println!("{:?}", values);
    // 消费者迭代器
    let v1_iter = values.iter();
    // sum 会拿走所有权
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // 迭代器适配器
    // 迭代器适配器，会返回一个新的迭代器，这是实现链式方法调用的关键：v.iter().map().filter()...
    // 需要一个消费者适配器来收尾，最终将迭代器转换成一个具体的值
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);


    //看一下复杂一点的例子
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        // 每两个元素剔除一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(idx, val)| val)
        // 累加 1+3+5 = 9
        .fold(0u64, |sum, acm| sum + acm);

    println!("{}", val);
}


