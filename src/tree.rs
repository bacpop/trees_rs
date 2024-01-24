use crate::gen_list::Mutation;
use crate::node::Node;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Tree {
    pub tree_vec: Vec<usize>,
    pub nodes: Vec<Node>,
    pub max_depth: usize,
    pub leaf_permutations: Vec<usize>,
    pub changes: HashMap<usize, Vec<usize>>,
    pub mutation_lists: Vec<Vec<Mutation>>,
}

// UTILITY FUNCTIONS FOR ADDING, ACCESSING, AND MUTATING NODES AND DATA IN NODES
impl Tree {
    // Constructor function for a new tree
    pub fn new(tree_vec: Vec<usize>) -> Tree {
        let k = tree_vec.len();
        Tree {
            tree_vec,
            nodes: vec![Node::default(); 2 * k + 1],
            max_depth: 0,
            leaf_permutations: (0..=k).collect(),
            changes: HashMap::new(),
            mutation_lists: Vec::new(),
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

    // Find the root of the tree
    pub fn get_root(&self) -> Option<&Node> {
        self.nodes.iter().find(|node| node.parent.is_none())
    }

    // Get the branch length from this node to its parent
    pub fn get_branchlength(&self, index: usize) -> f64 {
        self.get_node(index).unwrap().branch_length
    }

    // Find maximum node depth
    pub fn max_treedepth(&self) -> usize {
        self.nodes.iter().map(|node| node.depth).max().unwrap_or(0)
    }

    pub fn path_length(&self, index1: usize, index2: usize) -> usize {
        let mut temp: HashSet<usize> = HashSet::new();

        let x: Vec<usize> = self.iter(self.get_node(index1))
                                .chain(self.iter(self.get_node(index2)))
                                .map(|n| n.index)
                                .collect();

        for i in &x {
            match temp.get(&i) {
                Some(_) => temp.remove(&i),
                None => temp.insert(*i),
            };
        };

        temp.iter().len()
    }
}

// STRUCTS + TREE METHODS FOR POSTORDER TRAVERSAL OF TREE

// Start: go as far left as possible
// If in Left node, swap and go left
// If in Right node, go up to parent
#[derive(Debug)]
pub struct PostOrder<'a> {
    tree: &'a Tree,
    start_flag: bool,
    current_node: Option<&'a Node>,
    end_index: usize,
}

// Enum for handedness of child nodes
#[derive(Debug)]
pub enum Handedness {
    Left,
    Right,
}

// next() function for PostOrder iterator
impl<'a> Iterator for PostOrder<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_flag {
            self.current_node = self.tree.most_left_child(self.current_node);
            self.start_flag = false;
        } else {
            // If we return to start node, end iterator
            if self.current_node.unwrap().index == self.end_index {
                return None;
            }

            let ind = self.current_node.unwrap().index;
            match self.tree.get_handedness(ind) {
                Handedness::Left => {
                    self.current_node = self
                        .tree
                        .most_left_child(self.tree.swap_to_right_child(ind));
                }
                Handedness::Right => {
                    self.current_node = self.tree.get_parent(ind);
                }
            }
        }

        self.current_node
    }
}

// Tree methods + child node handedness functions
impl<'a> Tree {
    // Traverses tree in postorder starting at a given node
    pub fn postorder(&'a self, node: Option<&'a Node>) -> PostOrder {
        PostOrder {
            current_node: node,
            end_index: node.unwrap().index,
            tree: self,
            start_flag: true,
        }
    }

    // Traverses tree in postorder and excludes leaf nodes
    pub fn postorder_notips(&'a self, node: Option<&'a Node>) -> impl Iterator<Item = &'a Node> {
        self.postorder(node).filter(|node| !node.tip)
    }

    // Checks whether this is a left or right child of its parent
    pub fn get_handedness(&self, index: usize) -> Handedness {
        let (l, _) = self.get_parent(index).unwrap().children;

        if l == Some(index) {
            Handedness::Left
        } else {
            Handedness::Right
        }
    }

    // Starting a given node, this function goes as far down left-handed children as it can and
    // returns the node it lands on
    pub fn most_left_child(&'a self, node: Option<&'a Node>) -> Option<&Node> {
        let mut cur_node = node;
        let mut cur_left_child = cur_node.unwrap().children.0;

        while cur_left_child.is_some() {
            cur_node = self.get_node(cur_left_child.unwrap());
            cur_left_child = cur_node.unwrap().children.0;
        }

        cur_node
    }

    // Swaps to the right child of the parent of a given left child node
    pub fn swap_to_right_child(&self, index: usize) -> Option<&Node> {
        self.get_node(self.get_parent(index).unwrap().children.1.unwrap())
    }

    // Traverses tree in preorder starting at a given node
    pub fn preorder(&'a self, node: Option<&'a Node>) -> Preorder {
        Preorder {
            current_node: node,
            next_node: node,
            tree: self,
            return_nodes: vec![]
        }
    }

    pub fn newick(&self) -> String {
        let mut current_node: Option<&Node> = self.get_root();
        let mut next_node: Option<&Node>;
        let mut return_nodes: Vec<Option<&Node>> = Vec::new();
        let mut newick: Vec<String> = vec![String::from(";"), current_node.unwrap().index.to_string()];

        while current_node.is_some() {

            match current_node.unwrap().children {
                (Some(a), None) => {
                    next_node = self.get_node(a);
                    
                    newick.push(String::from(")"));
                    newick.push(next_node.unwrap().index.to_string());
                },
                (Some(a), Some(b)) => {
                    next_node = self.get_node(a);
                    return_nodes.push(self.get_node(b));
        
                    newick.push(String::from(")"));
                    newick.push(next_node.unwrap().index.to_string());
                },
                (None, _) => {
                    next_node = match return_nodes.pop() {
                        None => None,
                        Some(a) => a,
                    };
                    if next_node.is_some() {
                        let n: usize = current_node.unwrap().depth - next_node.unwrap().depth;
        
                        if n == 0 {
                            newick.push(String::from(","));
                        } else if n > 0 {
                            for _ in 1..=n {
                                newick.push(String::from("("));
                            }
                            newick.push(String::from(","));
                        }
        
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

// GRAVEYARD

// Return a mutable reference to the parent of a given node
// pub fn mut_parent(&mut self, index: usize) -> Option<&mut Node> {
//     match self.nodes.get(index).unwrap().parent {
//         Some(i) => self.mut_node(i),
//         None => None,
//     }
// }

#[derive(Debug)]
pub struct RootIter<'a> {
    current_node: Option<&'a Node>,
    next_node: Option<&'a Node>,
    tree: &'a Tree,
    end_flag: bool,
}

// Traverses from a specified node up to the root of the tree
impl<'a> Iterator for RootIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Self::Item>;

        if self.end_flag {
            return None;
        };

        match self.current_node.unwrap().parent {
            None => {
                output = self.tree.get_root();
                self.end_flag = true;
            }
            Some(i) => {
                output = self.current_node;
                self.next_node = self.tree.get_node(i);
            }
        };

        self.current_node = self.next_node;

        output
    }
}

impl<'a> Tree {
    // Iterates from a specified node upwards to the root of the tree
pub fn iter(&'a self, node: Option<&'a Node>) -> RootIter {
    RootIter {
        current_node: node,
        next_node: node,
        tree: self,
        end_flag: false,
    }
}

// Rootwards iterator that ignores leaves
pub fn iter_notips(&'a self, node: Option<&'a Node>) -> impl Iterator<Item = &'a Node> {
    self.iter(node).filter(|node| !node.tip)
}
}

#[derive(Debug)]
pub struct Preorder<'a> {
    current_node: Option<&'a Node>,
    next_node: Option<&'a Node>,
    return_nodes: Vec<Option<&'a Node>>,
    tree: &'a Tree,
    // pub newick: String,
}

// Traverses tree in preorder starting from specified node
impl<'a> Iterator for Preorder<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<&'a Node> = self.current_node;

        if self.current_node.is_none() {
            return output;
        }
        match self.current_node.unwrap().children {
            (Some(a), None) => {
                self.next_node = self.tree.get_node(a);
                // self.newick.push(')');
                // self.newick.push_str(&output.unwrap().index.to_string());
            }
            (Some(a), Some(b)) => {
                self.next_node = self.tree.get_node(a);
                self.return_nodes.push(self.tree.get_node(b));
                // self.newick.push('(');
                // self.newick.push_str(&self.next_node.unwrap().index.to_string());
                // self.newick.push(',');
                // new_newick.push(')');
            }
            (None, None) => {
                self.next_node = match self.return_nodes.pop() {
                    None => None,
                    Some(node) => node,
                };
                if self.next_node.is_some() {
                    match self.next_node.unwrap().parent {
                        Some(x) if x == output.unwrap().index => {
                            // new_newick.push('(');
                        },
                        Some(x) => {
                            // new_newick.push(',');
                        },
                        None => {

                        }
                    }
                } else {
                    // new_newick.push('(');
                }
            }
            _ => {
                panic!("Iterator has found a node with only a right child")
            }
        };

        self.current_node = self.next_node;

        // self.newick.push(new_newick);
        output
    }
}



// Traverses up to the root, updating likelihood as it goes
// pub fn update_likelihood_rootward(&'a self,
//     node: Option<&'a Node>,
//     genetic_data: &mut GeneticData,
//     rate_matrix: &na::Matrix4<f64>) {

//     for elem in self.iter_notips(node) {
//         let branchlengths = (self.get_branchlength(elem.children.0.unwrap()),
//         self.get_branchlength(elem.children.1.unwrap()));

//     let seq1 = genetic_data.likelihood_lists.get(elem.children.0.unwrap());
//     let seq2 = genetic_data.likelihood_lists.get(elem.children.1.unwrap());

//     genetic_data.likelihood_lists[elem.index] = combine_lists(seq1, seq2, branchlengths, rate_matrix);
//     }

// }

// Returns vector of nodes in tree that are tips
// pub fn get_tips(&self) -> Vec<&Node> {
//     self.nodes.iter().filter(|n| n.tip).collect()
// }
