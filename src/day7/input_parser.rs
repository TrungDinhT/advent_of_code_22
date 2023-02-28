use super::directory_tree::{Node, NodeArena};
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

fn parse_cd(current_id: &mut Option<usize>, node_arena: &mut NodeArena, dir_name: String) {
    if dir_name == ".." {
        if let Some(dir_id) = current_id {
            if let Node::DIR(dir) = node_arena.node(*dir_id) {
                *current_id = dir.parent_id;
            } else {
                panic!("This suppose to be a dir node");
            }
        }
    } else {
        let new_dir_id = node_arena.add_dir(*current_id, dir_name);
        *current_id = Some(new_dir_id);
    }
}

fn parse_ls(dir_content: &[String], node_arena: &mut NodeArena, current_dir_id: usize) {
    for file_str in dir_content.iter().filter(|s| !s.starts_with("dir")) {
        if let Some((size_str, name_str)) = file_str.split_whitespace().collect_tuple() {
            let size = size_str.parse().unwrap();
            let name = String::from(name_str);
            node_arena.add_file(current_dir_id, size, name);
        } else {
            panic!("Unknown format for entry: {}", file_str);
        }
    }
}

fn parse_cmd_results(cmd_results: Vec<Cmd>) -> NodeArena {
    let mut current_id: Option<usize> = None;
    let mut node_arena = NodeArena::new();
    for cmd_result in cmd_results {
        match cmd_result {
            Cmd::CD(dir_name) => parse_cd(&mut current_id, &mut node_arena, dir_name),
            Cmd::LS(dir_content) => {
                if let Some(current_dir_id) = current_id {
                    parse_ls(&dir_content, &mut node_arena, current_dir_id);
                } else {
                    panic!("Current dir id is None, but ls got file")
                }
            }
        }
    }

    node_arena
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_and_parse_input() {
        let cmd_results = read_and_split_by_command("src/day7/input_test.txt");
        for elm in cmd_results.iter() {
            match &elm {
                Cmd::CD(node_path) => println!("cd {}", node_path),
                Cmd::LS(ls_results) => println!("ls\n\t{}", ls_results.join("\n\t")),
            };
        }
        assert!(!cmd_results.is_empty());

        let node_arena = parse_cmd_results(cmd_results);
        for node in &node_arena.elements {
            println!("{:#?}", node);
        }
        assert!(node_arena.len() > 0);
    }
}
