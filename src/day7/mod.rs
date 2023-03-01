mod directory_tree;
mod input_parser;

use directory_tree::Node;
use input_parser::read_and_parse_input;

pub mod part1 {
    use super::*;

    pub fn sum_dir_size_at_most_100kb() -> usize {
        sum_dir_size_at_most_100kb_impl("src/day7/input.txt")
    }

    pub fn sum_dir_size_at_most_100kb_impl(path: &str) -> usize {
        let node_arena = read_and_parse_input(path);
        node_arena
            .elements
            .iter()
            .map(|node| {
                if let Node::DIR(dir_node) = node {
                    dir_node.size
                } else {
                    0
                }
            })
            .filter(|&size| size <= 100000)
            .sum()
    }
}

pub mod part2 {
    use super::*;
    
    pub fn smallest_dir_size_to_delete() -> usize {
        smallest_dir_size_to_delete_impl("src/day7/input.txt")
    }

    pub fn smallest_dir_size_to_delete_impl(path: &str) -> usize {
        const AVAILABLE_SIZE: usize = 70000000;
        const MIN_UNUSED_SIZE: usize = 30000000;

        let node_arena = read_and_parse_input(path);
        if let Node::DIR(root_dir) = node_arena.node(0) {
            let unused_size = AVAILABLE_SIZE - root_dir.size;
            if unused_size < MIN_UNUSED_SIZE {
                let size_to_free = MIN_UNUSED_SIZE - unused_size;
                return node_arena
                    .elements
                    .iter()
                    .map(|node| {
                        if let Node::DIR(dir_node) = node {
                            if dir_node.size >= size_to_free {
                                return dir_node.size;
                            }
                        }
                        MIN_UNUSED_SIZE
                    })
                    .min()
                    .unwrap();
            } else {
                return 0;
            }
        } else {
            panic!("Root is a directory");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part1::sum_dir_size_at_most_100kb_impl;
    use super::part2::smallest_dir_size_to_delete_impl;

    #[test]
    fn test_sum_dir_size_at_most_100kb() {
        let total_size = sum_dir_size_at_most_100kb_impl("src/day7/input_test.txt");
        assert_eq!(total_size, 95437);
    }

    #[test]
    fn test_smallest_dir_size_to_delete() {
        let size = smallest_dir_size_to_delete_impl("src/day7/input_test.txt");
        assert_eq!(size, 24933642);
    }
}
