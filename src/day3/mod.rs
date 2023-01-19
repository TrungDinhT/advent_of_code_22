use std::collections::HashSet;
use std::fs;

pub mod part2 {
    use itertools::Itertools;

    use super::*;

    pub fn sum_badges_priorities() -> u32 {
        sum_badges_priorities_impl("src/day3/input.txt")
    }

    fn sum_badges_priorities_impl(file_path: &str) -> u32 {
        fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(to_chars_set)
            .chunks(3)
            .into_iter()
            .map(|mut group_of_sacks| {
                let init_sack = group_of_sacks.next().unwrap();
                group_of_sacks
                    .fold(init_sack, |result, current| {
                        result.intersection(&current).copied().collect()
                    })
                    .into_iter()
                    .map(convert_to_priority)
                    .next()
                    .unwrap()
            })
            .sum()
    }

    #[test]
    fn sum_badges_priorities_impl_test() {
        assert_eq!(
            70,
            sum_badges_priorities_impl("src/day3/input_test_part2.txt")
        );
    }
}

pub mod part1 {
    use super::*;

    pub fn sum_shared_items_priorities() -> u32 {
        sum_shared_items_priorities_impl("src/day3/input.txt")
    }

    fn sum_shared_items_priorities_impl(file_path: &str) -> u32 {
        fs::read_to_string(file_path)
            .unwrap()
            .lines()
            .map(|rucksack| {
                let half = (rucksack.len() + 1) / 2; // ceiled division
                let first_compartment = to_chars_set(&rucksack[..half]);
                let second_compartment = to_chars_set(&rucksack[half..]);
                first_compartment
                    .intersection(&second_compartment)
                    .map(|&item| convert_to_priority(item))
                    .next()
                    .unwrap()
            })
            .sum()
    }

    #[test]
    fn sum_shared_items_priorities_impl_test() {
        assert_eq!(
            171,
            sum_shared_items_priorities_impl("src/day3/input_test_part1.txt")
        );
    }
}

fn to_chars_set(s: &str) -> HashSet<char> {
    s.chars().collect::<HashSet<char>>()
}

fn convert_to_priority(c: char) -> u32 {
    if c >= 'a' && c <= 'z' {
        1 + c as u32 - 'a' as u32
    } else if c >= 'A' && c <= 'Z' {
        27 + c as u32 - 'A' as u32
    } else {
        panic!("Unimplemented char: {}", c)
    }
}
