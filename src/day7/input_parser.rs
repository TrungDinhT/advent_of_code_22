use itertools::Itertools;
use std::fs;

enum Cmd {
    CD(String),
    LS(Vec<String>),
}

fn read_and_split_by_command(path: &str) -> Vec<Cmd> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .split("$")
        .skip(1) // the text start with $, so we skip the first elm split
        .map(|s| {
            let s = s.trim();
            if s.starts_with("cd") {
                let (_cd, path) = s.split_whitespace().collect_tuple().unwrap();
                Cmd::CD(String::from(path))
            } else if s.starts_with("ls") {
                // skip first line because it's `ls`
                Cmd::LS(s.lines().skip(1).map(String::from).collect())
            } else {
                panic!("Unknown command: {}", s);
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_split_by_command() {
        let output = read_and_split_by_command("src/day7/input_test.txt");
        for elm in output.iter() {
            match &elm {
                Cmd::CD(node_path) => println!("cd {}", node_path),
                Cmd::LS(ls_results) => println!("ls\n\t{}", ls_results.join("\n\t")),
            };
        }
        assert!(!output.is_empty());
    }
}
