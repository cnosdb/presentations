

fn main(){
    /// 无处不在的 if else
    let n = 6;
    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    println!("The value of number is: {}", n);

    ///  for 循环
    ///  for item in collection	          for item in IntoIterator::into_iter(collection)	转移所有权
    ///  for item in &collection  	      for item in collection.iter()	                    不可变借用
    ///  for item in &mut collection	  for item in collection.iter_mut()	                可变借用

    /// 第一种使用方式中 collection[index] 的索引访问，
    /// 会因为边界检查(Bounds Checking)导致运行时的性能损耗
    let mut collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        println!("item: {}", item);
    }
    // 推荐使用下面这种方式进行遍历
    for item in  &mut collection.iter_mut() {
        *item = *item + 1;
    }

    let mut n = 0;
    while n <= 5  {
        println!("num n: {}", n);
        n = n + 1;
    }


    /// break 可以单独使用，也可以带一个返回值，有些类似 return
    /// loop 是一个表达式，因此可以返回一个值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}