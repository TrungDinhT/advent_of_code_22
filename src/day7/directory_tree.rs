struct NodeArena {
    elements: Vec<Node>,
}

struct DirNode {
    id: usize,
    parent_id: Option<usize>,
}

struct FileNode {
    id: usize,
    parent_id: usize,
    size: FileSize,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct FileSize(usize);

enum Node {
    DIR(DirNode),
    FILE(FileNode),
}

impl NodeArena {
    fn new() -> Self {
        NodeArena { elements: vec![] }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn node(&self, id: usize) -> &Node {
        &self.elements[id]
    }

    fn add_dir(&mut self, parent_id: Option<usize>) -> usize {
        let id = self.elements.len();
        self.elements.push(Node::DIR(DirNode { id, parent_id }));
        id
    }

    fn add_file(&mut self, parent_id: usize, size: FileSize) -> usize {
        let id = self.elements.len();
        self.elements.push(Node::FILE(FileNode {
            id,
            parent_id,
            size,
        }));
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_construct_node_arena() {
        let mut node_arena = NodeArena::new();

        let root_node_id = node_arena.add_dir(None);
        assert_eq!(node_arena.len(), 1);

        if let Node::DIR(root_node) = node_arena.node(root_node_id) {
            assert_eq!(root_node.id, 0);
            assert_eq!(root_node.parent_id, None);

            let second_dir_id = node_arena.add_dir(Some(root_node.id));
            assert_eq!(node_arena.len(), 2);

            if let Node::DIR(second_dir) = node_arena.node(second_dir_id) {
                assert_eq!(second_dir.id, 1);
                assert_eq!(second_dir.parent_id, Some(0));

                let file_size = FileSize(100);
                let file_id = node_arena.add_file(second_dir_id, file_size);
                assert_eq!(node_arena.len(), 3);

                if let Node::FILE(file) = node_arena.node(file_id) {
                    assert_eq!(file.id, 2);
                    assert_eq!(file.parent_id, second_dir_id);
                    assert_eq!(file.size, file_size);
                } else {
                    assert!(false, "this has to be a file node");
                }
            } else {
                assert!(false, "second_dir is a dir node");
            }
        } else {
            assert!(false, "root_node is a dir");
        }
    }
}
