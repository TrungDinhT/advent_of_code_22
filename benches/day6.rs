use criterion::Criterion;
use std::fs;

fn benchmark_day6<F>(c: &mut Criterion, f: F)
where
    F: Fn(&str) -> usize,
{
    let buffer = fs::read_to_string("src/day6/input.txt").unwrap();
    c.bench_function("find_marker_pos", |b| b.iter(|| f(&buffer)));
}

pub fn benchmark_day6_part1_impl1(c: &mut Criterion) {
    use advent_of_code_22::day6;
    benchmark_day6(c, day6::part1::impl1::find_marker_pos_impl);
}

pub fn benchmark_day6_part1_impl2(c: &mut Criterion) {
    use advent_of_code_22::day6;
    benchmark_day6(c, day6::part1::impl2::find_marker_pos_impl);
}
