use crate::day7::input_parser::Cmd::{CD, LS};
use itertools::Itertools;
use std::fs;

struct NodePath;
struct NodeContent;

enum Cmd {
    CD(String),
    LS(Vec<String>),
}

fn read_and_split_by_command(path: &str) -> Vec<Cmd> {
    let input_str = fs::read_to_string(path).unwrap();
    let content = input_str
        .trim()
        .split("$")
        .skip(1) // the text start with $, so we skip the first elm split
        .map(|s| {
            let s = s.trim();
            if s.starts_with("cd") {
                let (_cd, path) = s.split_whitespace().collect_tuple().unwrap();
                CD(String::from(path))
            } else if s.starts_with("ls") {
                // skip first line because it's `ls`
                LS(s.lines().skip(1).map(String::from).collect())
            } else {
                panic!("Unknown command: {}", s);
            }
        });
    for elm in content {
        match elm {
            CD(node_path) => println!("cd {}", node_path),
            LS(ls_results) => println!("ls\n\t{}", ls_results.join("\n\t")),
        };
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use crate::day7::input_parser::read_and_split_by_command;

    #[test]
    fn test_read_and_split_by_command() {
        let output = read_and_split_by_command("src/day7/input_test.txt");
        assert!(!output.is_empty());
    }
}
