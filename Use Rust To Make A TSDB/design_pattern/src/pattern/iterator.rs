use std::collections::HashSet;
use std::iter::Iterator;
use std::ops::AddAssign;


// [)
pub struct NumberIterator {
    begin: usize,
    end: usize
}

impl NumberIterator {
    pub fn new (begin: usize, end: usize) -> Self {
        Self {
            begin,
            end
        }

    }
}
// 0, 1, 2, 3..99
impl Iterator for NumberIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.begin < self.end {
            let res = self.begin;
            self.begin.add_assign(1);
            Some(res)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn count_iterator(mut iterator:  impl Iterator<>) -> usize {
    let mut res = 0;
    while let Some(_) = iterator.next() {
        res.add_assign(1);
    }
    res
}

#[test]
fn test_filter_iterator()  {

    (0..5) //初始的迭代器
        .filter(|n| n % 2 == 0) // 过滤的迭代器
        .for_each(|n| println!("{n}")) //对迭代器每个元素进行操作

}

#[test]
fn test_map_iterator() {
    (0..5)
        .map(|n| n as f64)
        .map(|n| n.sqrt())
        .for_each(|n| println!("{n}"))

}


#[test]
fn test_number_iterator() {
    let a = count_iterator(0..5);
    println!("{a}");
}

#[test]
fn test_number_sum() {
    println!("{}", (0..5).sum::<usize>())
}

#[test]
fn test_fold() {
    //init: 0
    // 0, 1, 2, 3, 4
    // init: 0
    // 1, 2, 3, 4
    // init: 1
    // 2, 3, 4
    // init: 3
    // 3, 4
    // init: 6
    // 4
    // init: 10

    let sum = (0..5).fold(0, |init, n| {
        init + n
    });
    println!("{sum}");

}

#[test]
fn test_collect() {
    let vec = (0..5).collect::<Vec<usize>>();
    assert_eq!(vec, vec![0, 1, 2, 3, 4]);


    let res = (0..5)
        .map(|n| if n == 3 {
            println!("{n}");
            Some(n)
        } else {
            None
        }).collect::<Option<Vec<usize>>>();

    assert_eq!(res, None)


}

#[test]
fn test_collect_hash_set() {
    (0..5)
        .map(|n| if n > 3 {
            3
        } else {
            n
        })
        .collect::<HashSet<usize>>()
        .iter()
        .for_each(|n| println!("{n}"));
}

#[test]
fn test_flatten() {
    // vec![[1, 2, 3], [4, 5, 6]]
    //     .into_iter()
    //     .flatten()
    //     .collect::<Vec<usize>>()
    //     .iter()
    //     .for_each(|n| println!("{n}"));

    println!("{:?}", Some(Some(1)).flatten());

    vec![Some(1), None, Some(2)]
        .into_iter()
        .flatten()
        .collect::<Vec<usize>>()
        .into_iter()
        .for_each(|n| {
            println!("{n}")
        })
}