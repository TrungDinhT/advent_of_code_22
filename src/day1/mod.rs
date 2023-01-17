use itertools::Itertools;
use std::fs;

fn calories_carried_per_elf<'a>(inputs: &'a str) -> impl Iterator<Item = u32> + 'a {
    inputs.split("\n\n").map(|inventory_per_elf| {
        inventory_per_elf
            .split("\n")
            .map(|calories| calories.parse::<u32>().unwrap())
            .sum()
    })
}

pub fn most_carried_calories() -> u32 {
    let inputs = fs::read_to_string("src/day1/input.txt").unwrap();
    calories_carried_per_elf(&inputs).max().unwrap()
}

pub fn sum_top3_most_carried_calories() -> u32 {
    let inputs = fs::read_to_string("src/day1/input.txt").unwrap();
    calories_carried_per_elf(&inputs)
        .sorted()
        .rev()
        .take(3)
        .sum()
}
