use std::{cell::RefCell, rc::Rc};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
fn set_rc_refcell(r: Rc<RefCell<i32>>, v: i32) {
    *r.borrow_mut() = v;
}

#[inline]
fn get_rc_refcell(r: Rc<RefCell<i32>>) -> i32 {
    *r.borrow()
}

fn bench_set_rc_refcell(c: &mut Criterion) {
    let data = Rc::new(RefCell::new(black_box(1)));

    c.bench_function("set_rc_refcell", |b| {
        b.iter(|| {
            get_rc_refcell(data.clone());
            set_rc_refcell(data.clone(), black_box(2));
            get_rc_refcell(data.clone());
            set_rc_refcell(data.clone(), black_box(2));
            get_rc_refcell(data.clone());
        })
    });
}

criterion_group!(benches, bench_set_rc_refcell);
criterion_main!(benches);
