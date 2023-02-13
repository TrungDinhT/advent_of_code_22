use criterion::{criterion_group, Criterion};
use std::fs;

fn benchmark_day6<F>(c: &mut Criterion, f: F, block_len: usize)
where
    F: Fn(&str, usize) -> usize,
{
    let buffer = fs::read_to_string("src/day6/input.txt").unwrap();
    c.bench_function("find_marker_pos", |b| b.iter(|| f(&buffer, block_len)));
}

fn benchmark_day6_part1_impl1(c: &mut Criterion) {
    use super::implementation::impl1::find_marker_pos_impl;
    benchmark_day6(c, find_marker_pos_impl, 4);
}

fn benchmark_day6_part1_impl2(c: &mut Criterion) {
    use super::implementation::impl2::find_marker_pos_impl;
    benchmark_day6(c, find_marker_pos_impl, 4);
}

fn benchmark_day6_part2_impl1(c: &mut Criterion) {
    use super::implementation::impl1::find_marker_pos_impl;
    benchmark_day6(c, find_marker_pos_impl, 14);
}

fn benchmark_day6_part2_impl2(c: &mut Criterion) {
    use super::implementation::impl2::find_marker_pos_impl;
    benchmark_day6(c, find_marker_pos_impl, 14);
}

criterion_group!(
    benches_day6,
    benchmark_day6_part1_impl1,
    benchmark_day6_part1_impl2,
    benchmark_day6_part2_impl1,
    benchmark_day6_part2_impl2,
);
