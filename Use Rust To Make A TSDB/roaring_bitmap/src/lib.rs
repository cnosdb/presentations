mod container;

use std::collections::HashMap;

use container::Container;

trait Set {
    type Item;
    fn insert(&mut self, val: Self::Item);
    fn remove(&mut self, val: Self::Item);
    fn contains(&self, val: Self::Item) -> bool;
}

struct RoaringBitmap {
    containers: HashMap<u16, Container>,
}

impl RoaringBitmap {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
        }
    }
}

impl Set for RoaringBitmap {
    type Item = u32;

    fn insert(&mut self, val: Self::Item) {
        let (key, index) = split(val);
        let container = self.containers.entry(key).or_insert(Container::new());
        container.insert(index);
    }

    fn remove(&mut self, val: Self::Item) {
        let (key, index) = split(val);
        if let Some(container) = self.containers.get_mut(&key) {
            container.remove(index);
        }
    }

    fn contains(&self, val: Self::Item) -> bool {
        let (key, index) = split(val);
        match self.containers.get(&key) {
            Some(c) => c.contains(index),
            None => false,
        }
    }
}

fn split(val: u32) -> (u16, u16) {
    ((val >> 16) as u16, val as u16)
}

#[test]
fn test1() {
    let mut rb = RoaringBitmap::new();
    let values = (0..1000000).map(|_| rand::random()).collect::<Vec<_>>();
    for v in values.iter().cloned() {
        rb.insert(v);
    }
    for v in values.iter().cloned() {
        assert!(rb.contains(v));
    }
    for &v in &values[500..] {
        rb.remove(v);
    }
    for &v in &values[..500] {
        assert!(rb.contains(v));
    }
    for &v in &values[500..] {
        assert!(!rb.contains(v));
    }

    let mut rb = RoaringBitmap::new();
    for v in 0..1000000 {
        rb.insert(v);
    }
    for v in 0..1000000 {
        assert!(rb.contains(v));
    }

    for v in 500..1000000 {
        rb.remove(v);
    }
    for v in 0..500 {
        assert!(rb.contains(v));
    }
    for v in 500..1000000 {
        assert!(!rb.contains(v));
    }
}
