use criterion::{criterion_group, criterion_main};

mod day6;

criterion_group!(
    benches,
    day6::benchmark_day6_part1_impl1,
    day6::benchmark_day6_part1_impl2
);

criterion_main!(benches);
