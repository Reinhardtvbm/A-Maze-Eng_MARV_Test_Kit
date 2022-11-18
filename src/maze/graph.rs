use std::collections::LinkedList;

pub struct Graph<T> {
    adjacency_list: Vec<LinkedList<T>>,
    adjacency_matrix: Vec<Vec<bool>>,
    nodes: Vec<T>,
    size: u32,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Self {
            adjacency_list: Vec::new(),
            adjacency_matrix: Vec::new(),
            nodes: Vec::new(),
            size: 0,
        }
    }

    pub fn add_node(&mut self, new_node: T) {
        // add the new node to the nodes vector
        self.nodes.push(new_node);

        self.size += 1;

        // if the adjacency matrix is empty, then
        // we create a new vector of size 1
        if self.adjacency_matrix.is_empty() {
            let mut new_row = Vec::new();
            new_row.push(false);

            self.adjacency_matrix.push(new_row);
        } else {
            let mut new_row = Vec::new();

            for row in &mut self.adjacency_matrix {
                row.push(false);
                new_row.push(false);
            }

            new_row.push(false);

            self.adjacency_matrix.push(new_row);
        }

        let new_list = LinkedList::new();
        self.adjacency_list.push(new_list);

        for row in &self.adjacency_matrix {
            print!("[");
            for cell in row {
                print!(" {} ", cell);
            }
            println!("]");
        }
    }

    pub fn get_node(&self, index: usize) -> Option<&T> {
        self.nodes.get(index)
    }
}
