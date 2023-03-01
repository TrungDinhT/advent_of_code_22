use super::directory_tree::NodeArena;
use super::input_parser::read_and_parse_input;
use criterion::{criterion_group, Criterion};

fn benchmark_day7_solutions<F>(c: &mut Criterion, f: F, func_name: &str)
where
    F: Fn(&NodeArena) -> usize,
{
    let node_arena = read_and_parse_input("src/day7/input.txt");
    c.bench_function(func_name, |b| b.iter(|| f(&node_arena)));
}

fn benchmark_day7_part1_solution(c: &mut Criterion) {
    use super::part1::sum_dir_size_at_most_100kb_impl;
    benchmark_day7_solutions(
        c,
        sum_dir_size_at_most_100kb_impl,
        "sum_dir_size_at_most_100kb_impl",
    );
}

fn benchmark_day7_part2_solution(c: &mut Criterion) {
    use super::part2::smallest_dir_size_to_delete_impl;
    benchmark_day7_solutions(
        c,
        smallest_dir_size_to_delete_impl,
        "smallest_dir_size_to_delete_impl",
    );
}

fn benchmark_day7_read_and_parse_input(c: &mut Criterion) {
    c.bench_function("read_and_parse_input", |b| {
        b.iter(|| read_and_parse_input("src/day7/input.txt"))
    });
}

criterion_group!(
    benches_day7,
    benchmark_day7_part1_solution,
    benchmark_day7_part2_solution,
    benchmark_day7_read_and_parse_input,
);
