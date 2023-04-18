// TODO: incomplete and unused

struct Node<T> {
    pub data: T,

    pub parent_index: usize,
    pub child_indices: Vec<usize>,
}

struct Tree<T> {
    nodes: Vec<Option<Node<T>>>,
}

impl<T> Tree<T> {
    /// Creates a new empty tree
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Sets the root node of the tree
    pub fn set_root_node(&mut self, data: T) {
        self.nodes.clear();
        self.nodes.push(Some(Node {
            data,
            parent_index: 0,
            child_indices: Vec::new(),
        }));
    }

    /// Adds a child to an existing node
    pub fn add_child(&mut self, node_index: usize, data: T) -> Result<(), ()> {
        Ok(())
    }

    /// Adds a node to the tree, returns the index of the new node
    fn push_node(&mut self, node: Node<T>) -> usize {
        todo!()
    }

    fn remove_node(&mut self, index: usize) -> Result<(), ()> {
        todo!()
    }
}
