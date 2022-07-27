use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn test_add(i: i32) -> i32 {
    i + 1
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test name plz", |b| b.iter(|| test_add(black_box(1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
