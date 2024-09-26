use crate::rate_matrix::RateMatrix;
use crate::Tree;
use crate::node::Node;
// Builds a newick string from a Tree struct
impl<T: RateMatrix> Tree<T> {
    pub fn newick(&self) -> String {
        let mut current_node: Option<&Node> = self.get_root();
        let mut next_node: Option<&Node>;
        let mut return_nodes: Vec<Option<&Node>> = Vec::new();
        let mut newick: Vec<String> = vec![String::from(";"), current_node.unwrap().branch_length.to_string(),
        String::from(":"), current_node.unwrap().index.to_string()];

        while current_node.is_some() {

            match current_node.unwrap().children {
                (Some(a), None) => {
                    next_node = self.get_node(a);

                    newick.push(String::from(")"));
                    newick.push(next_node.unwrap().branch_length.to_string());
                    newick.push(String::from(":"));
                    newick.push(next_node.unwrap().index.to_string());

                },
                (Some(a), Some(b)) => {
                    next_node = self.get_node(a);
                    return_nodes.push(self.get_node(b));

                    newick.push(String::from(")"));
                    newick.push(next_node.unwrap().branch_length.to_string());
                    newick.push(String::from(":"));
                    newick.push(next_node.unwrap().index.to_string());

                },
                (None, _) => {
                    next_node = match return_nodes.pop() {
                        None => None,
                        Some(a) => a,
                    };
                    if next_node.is_some() {
                        let n: usize = current_node.unwrap().depth - next_node.unwrap().depth;

                        match n {
                            0 => {newick.push(String::from(","));},
                            _ => {
                                for _ in 1..=n {
                                    newick.push(String::from("("));
                                }
                                newick.push(String::from(","));
                            },
                        }

                        newick.push(next_node.unwrap().branch_length.to_string());
                        newick.push(String::from(":"));
                        newick.push(next_node.unwrap().index.to_string());
                    } else {
                        let n: usize = current_node.unwrap().depth;
                        for _ in 1..=n {
                            newick.push(String::from("("));
                        }
                    }
                }
            }
            
            current_node = next_node;
        }
        newick.reverse();
        newick.concat()
    }
}