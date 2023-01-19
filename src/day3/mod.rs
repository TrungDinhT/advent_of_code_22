use std::collections::HashSet;
use std::fs;

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

mod test {
    #[test]
    fn sum_shared_items_priorities_impl_test() {
        assert_eq!(
            171,
            super::sum_shared_items_priorities_impl("src/day3/input_test.txt")
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
