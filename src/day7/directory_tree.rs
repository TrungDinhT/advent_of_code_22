pub struct NodeArena {
    pub elements: Vec<Node>,
}

#[derive(Debug)]
pub struct DirNode {
    pub id: usize,
    pub parent_id: Option<usize>,
    pub name: String,
    pub size: usize,
}

#[derive(Debug)]
pub struct FileNode {
    pub id: usize,
    pub parent_id: usize,
    pub size: usize,
    pub name: String,
}

#[derive(Debug)]
pub enum Node {
    DIR(DirNode),
    FILE(FileNode),
}

impl NodeArena {
    pub fn new() -> Self {
        NodeArena { elements: vec![] }
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn node(&self, id: usize) -> &Node {
        &self.elements[id]
    }

    pub fn add_dir(&mut self, parent_id: Option<usize>, name: String) -> usize {
        let id = self.elements.len();
        let size = 0;
        self.elements.push(Node::DIR(DirNode {
            id,
            parent_id,
            name,
            size,
        }));
        id
    }

    fn add_size_to_dir(&mut self, dir_id: usize, size: usize) {
        if let Node::DIR(dir) = &mut self.elements[dir_id] {
            dir.size = dir.size + size;
            if let Some(parent_id) = dir.parent_id {
                self.add_size_to_dir(parent_id, size);
            }
        }
    }

    pub fn add_file(&mut self, parent_id: usize, size: usize, name: String) -> usize {
        let id = self.elements.len();
        self.elements.push(Node::FILE(FileNode {
            id,
            parent_id,
            size,
            name,
        }));
        self.add_size_to_dir(parent_id, size);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_construct_node_arena() {
        let mut node_arena = NodeArena::new();

        let root_node_name = String::from("root_node");
        let root_node_id = node_arena.add_dir(None, root_node_name.clone());
        assert_eq!(node_arena.len(), 1);

        if let Node::DIR(root_node) = node_arena.node(root_node_id) {
            assert_eq!(root_node.id, 0);
            assert_eq!(root_node.parent_id, None);
            assert_eq!(root_node.name, root_node_name);
            assert_eq!(root_node.size, 0);

            let second_dir_name = String::from("second_dir");
            let second_dir_id = node_arena.add_dir(Some(root_node.id), second_dir_name.clone());
            assert_eq!(node_arena.len(), 2);

            if let Node::DIR(second_dir) = node_arena.node(second_dir_id) {
                assert_eq!(second_dir.id, 1);
                assert_eq!(second_dir.parent_id, Some(0));
                assert_eq!(second_dir.name, second_dir_name);
                assert_eq!(second_dir.size, 0);

                let file_size = 100;
                let file_name = String::from("file.txt");
                let file_id = node_arena.add_file(second_dir_id, file_size, file_name.clone());
                assert_eq!(node_arena.len(), 3);

                if let Node::FILE(file) = node_arena.node(file_id) {
                    assert_eq!(file.id, 2);
                    assert_eq!(file.parent_id, second_dir_id);
                    assert_eq!(file.size, file_size);
                    assert_eq!(file.name, file_name);
                } else {
                    assert!(false, "this has to be a file node");
                }

                if let Node::DIR(second_dir) = node_arena.node(second_dir_id) {
                    assert_eq!(second_dir.size, 100);
                }

                if let Node::DIR(root_node) = node_arena.node(root_node_id) {
                    assert_eq!(root_node.size, 100);
                }
            } else {
                assert!(false, "second_dir is a dir node");
            }
        } else {
            assert!(false, "root_node is a dir");
        }
    }
}
