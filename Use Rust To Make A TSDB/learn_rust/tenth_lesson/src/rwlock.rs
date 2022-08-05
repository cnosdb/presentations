use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);

    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }

    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);

        // let r1 = lock.read();
        // println!("{:?}",r1);
    }
}
