use itertools::Itertools;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

pub mod part2 {
    use super::*;

    pub fn nb_pairs_overlapped() -> usize {
        nb_pairs_overlapped_impl("src/day4/input.txt")
    }

    fn nb_pairs_overlapped_impl(file_path: &str) -> usize {
        nb_pairs_problematic_impl(file_path, Range::overlapped)
    }

    #[test]
    fn test_nb_pairs_fully_overlapped_impl() {
        assert_eq!(4, nb_pairs_overlapped_impl("src/day4/input_test.txt"));
    }
}

pub mod part1 {
    use super::*;

    pub fn nb_pairs_fully_overlapped() -> usize {
        nb_pairs_fully_overlapped_impl("src/day4/input.txt")
    }

    fn nb_pairs_fully_overlapped_impl(file_path: &str) -> usize {
        nb_pairs_problematic_impl(file_path, Range::fully_overlapped)
    }

    #[test]
    fn test_nb_pairs_fully_overlapped_impl() {
        assert_eq!(2, nb_pairs_fully_overlapped_impl("src/day4/input_test.txt"));
    }
}

fn nb_pairs_problematic_impl<P>(file_path: &str, predicate: P) -> usize
where
    P: Fn(&Range, &Range) -> bool,
{
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .filter(|pair| {
            let (first, second): (Range, Range) = split_in_two(pair, ',').unwrap();
            predicate(&first, &second)
        })
        .count()
}

struct Range {
    start: u8,
    end: u8,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(range_str: &str) -> Result<Self, Self::Err> {
        let (start, end) = split_in_two(range_str, '-')?;
        Ok(Range { start, end })
    }
}

impl Range {
    fn fully_overlapped(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (self.start >= other.start && self.end <= other.end)
    }

    fn overlapped(&self, other: &Self) -> bool {
        self.end >= other.start && self.start <= other.end
    }
}

fn split_in_two<T, E>(s: &str, sep: char) -> Result<(T, T), E>
where
    T: FromStr<Err = E>,
{
    if let Some((first_str, second_str)) = s.split(sep).collect_tuple() {
        let first = T::from_str(first_str)?;
        let second = T::from_str(second_str)?;
        Ok((first, second))
    } else {
        panic!("Cannot split '{}' in two parts separated by '{}'", s, sep)
    }
}
