use criterion::{black_box, criterion_group, criterion_main, Criterion};
use practice::leecode::lc0058;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lc0004-1", |b| {
        b.iter(|| lc0058::Solution::length_of_last_word(black_box("a b".to_string())))
    });
    c.bench_function("lc0058-2", |b| {
        b.iter(|| lc0058::Solution::length_of_last_word2(black_box("a b".to_string())))
    });
    c.bench_function("lc0058-3", |b| {
        b.iter(|| lc0058::Solution::length_of_last_word_chars(black_box("a b".to_string())))
    });
    c.bench_function("lc0058-4", |b| {
        b.iter(|| lc0058::Solution::length_of_last_word_loop(black_box("a b".to_string())))
    });
    c.bench_function("lc0058-5", |b| {
        b.iter(|| lc0058::Solution::length_of_last_word_split(black_box("a b".to_string())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
