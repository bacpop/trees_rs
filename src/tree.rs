use crate::mutation::{create_list, Mutation};
use crate::node::Node;
use crate::rate_matrix::{RateMatrix};
use crate::vector_to_tree;
use needletail::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Tree <T: RateMatrix> {
    pub tree_vec: Vec<usize>,
    pub nodes: Vec<Node>,
    pub max_depth: usize,
    pub label_dictionary: HashMap<usize, String>,
    pub changes: HashMap<usize, Vec<usize>>,
    pub mutation_lists: Vec<Vec<Mutation>>,
    pub rate_matrix: T,
}

// Tree methods
impl<T: RateMatrix> Tree<T> {
    // Constructor function for a new tree
    pub fn new(tree_vec: &[usize], rate_matrix: T) -> Self {
        let k = tree_vec.len();
        let n_nodes: usize = 2 * k + 1;
        Tree {
            tree_vec: tree_vec.to_vec(),
            nodes: vec![Node::default(); n_nodes],
            max_depth: 0,
            label_dictionary: HashMap::new(),
            changes: HashMap::new(),
            mutation_lists: Vec::with_capacity(n_nodes),
            rate_matrix,
        }
    }

    // Add a node to the tree
    pub fn add(&mut self, index: usize, parent: Option<usize>) {
        let mut dpth: usize = 0;

        if let Some(par) = parent {
            self.mut_node(par).unwrap().new_child(index);
            dpth = self.get_node(par).unwrap().depth + 1;
        }

        self.nodes[index] = Node::new(parent, (None, None), index, dpth, 1.0);
    }

    // Update a Tree to a new integer vector
    pub fn update(&mut self, new_vec: &[usize]) {
        let new_tree: Tree<T> = vector_to_tree(new_vec, &self.rate_matrix);
        let k: usize = new_tree.nodes.len();
        let mut old_parent: Option<usize>;
        let mut new_parent: Option<usize>;

        for i in (0..k).rev() {
            old_parent = self.get_node(i).unwrap().parent;
            new_parent = new_tree.get_node(i).unwrap().parent;

            if old_parent.ne(&new_parent) {
                let d = new_tree.get_node(i).unwrap().depth;

                match self.changes.get(&d) {
                    None => {
                        self.changes.insert(d, vec![i]);
                    }
                    Some(_) => {
                        self.changes.get_mut(&d).unwrap().push(i);
                    }
                }
            }
        }

        self.tree_vec = new_tree.tree_vec;
        self.nodes = new_tree.nodes;
    }

    pub fn add_genetic_data(&mut self, filename: &str) {
        let mut reader = parse_fastx_file(filename).expect("Error parsing file");

        // Add genetic data
        while let Some(rec) = reader.next() {
            let newrec: Vec<char> = rec.unwrap().seq().iter().map(|l| *l as char).collect();
            self.mutation_lists.push(create_list(&newrec));
        }

        // Add empty lists for internal nodes
        let leafn = self.mutation_lists.len() - 1;
        for _ in 0..leafn {
            self.mutation_lists.push(Vec::new());
        }
    }

    // Get a specified node
    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    // Return a mutable reference to a given node
    pub fn mut_node(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    // Get parent of specified node
    pub fn get_parent(&self, index: usize) -> Option<&Node> {
        match self.nodes.get(index).unwrap().parent {
            Some(i) => self.get_node(i),
            None => None,
        }
    }

    // Count the leaves in a Tree
    pub fn count_leaves(&self) -> usize {
        self.nodes.iter().filter(|node| node.tip).count()
    }

    // Find the root of the tree
    pub fn get_root(&self) -> Option<&Node> {
        self.nodes.iter().find(|node| node.parent.is_none())
    }

    // Get the branch length from this node to its parent
    pub fn get_branchlength(&self, index: usize) -> f64 {
        self.get_node(index).unwrap().branch_length
    }

    pub fn get_branchlengths(&self) -> nalgebra::Vector1<f64> {
        let out: Vec<f64> = self.nodes.iter().map(|node| node.branch_length).collect();
        nalgebra::Vector1::from_vec(out)
    }

    // Find maximum node depth
    pub fn max_treedepth(&self) -> usize {
        self.nodes.iter().map(|node| node.depth).max().unwrap_or(0)
    }

}
