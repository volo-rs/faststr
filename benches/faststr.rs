use criterion::{black_box, criterion_group, criterion_main, Criterion};
use faststr::FastStr;

fn criterion_benchmark(c: &mut Criterion) {
    let s = FastStr::empty();
    c.bench_function("empty faststr", |b| b.iter(|| black_box(s.clone())));
    let s = String::new();
    c.bench_function("empty string", |b| b.iter(|| black_box(s.clone())));

    let s = FastStr::from("Hello, world!");
    c.bench_function("static faststr", |b| b.iter(|| black_box(s.clone())));
    let s = FastStr::new_inline("Hello, world!");
    c.bench_function("inline faststr", |b| b.iter(|| black_box(s.clone())));
    let s = String::from("Hello, world!");
    c.bench_function("string hello world", |b| b.iter(|| black_box(s.clone())));

    for size in [512, 4 * 1024, 16 * 1024, 64 * 1024, 512 * 1024, 1024 * 1024] {
        let s = FastStr::from("a".repeat(size));
        let _s1 = black_box(s.clone());
        let _s2 = black_box(s.clone());
        c.bench_function(format!("{}B faststr", size).as_str(), |b| {
            b.iter(|| black_box(s.clone()))
        });
        drop(_s1);
        drop(_s2);
        let s = String::from("a".repeat(size));
        c.bench_function(format!("{}B string", size).as_str(), |b| {
            b.iter(|| black_box(s.clone()))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
