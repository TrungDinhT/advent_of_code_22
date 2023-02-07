use itertools::Itertools;
use std::fs;

mod action;
mod tests;
mod warehouse;

pub mod part1 {
    use super::action::Action;
    use super::warehouse::Warehouse;
    use super::*;

    pub fn top_crates_after_rearrangement() -> String {
        top_crates_after_rearrangement_impl("src/day5/input.txt")
    }

    fn top_crates_after_rearrangement_impl(path: &str) -> String {
        if let Some((warehouse_str, actions_str)) = fs::read_to_string(path)
            .unwrap()
            .trim_end()
            .split("\n\n")
            .collect_tuple()
        {
            let mut warehouse: Warehouse = warehouse_str.parse().unwrap();
            for action_str in actions_str.lines() {
                let Action { n_moved, from, to } = action_str.parse().unwrap();
                warehouse.move_crates_between_stacks(n_moved, from, to);
            }
            warehouse.top_of_stacks()
        } else {
            panic!("Parsing input failed!")
        }
    }

    #[test]
    fn test_top_crates_after_rearrangement_impl() {
        const TOP_CRATES_REF: &str = "CMZ";
        let top_crates = top_crates_after_rearrangement_impl("src/day5/input_test.txt");
        assert_eq!(top_crates, TOP_CRATES_REF);
    }
}
