use std::thread::current;

use clap::builder::NonEmptyStringValueParser;
use ndarray::AssignElem;

use crate::node::Node;
use crate::rate_matrix::RateMatrix;
use crate::tree::Tree;

//////////////////////////////
// Post-order Tree iterator //
//////////////////////////////

// Start: go as far left as possible
// If in Left node, swap and go left
// If in Right node, go up to parent
#[derive(Debug)]
pub struct PostOrder<'a, T: RateMatrix> {
    tree: &'a Tree<T>,
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
impl<'a, T: RateMatrix> Iterator for PostOrder<'a, T> {
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

// Tree methods used in post-order traversal
impl<'a, T: RateMatrix> Tree<T> {
    // Traverses tree in postorder starting at a given node
    pub fn postorder(&'a self, node: Option<&'a Node>) -> PostOrder<T> {
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
}

// Changes iterator
// #[derive(Debug)]
// pub struct ChangeOrder<'a, T: RateMatrix> {
//     tree: &'a mut Tree<T>,
//     current_vec: Vec<usize>,
// }

// impl<'a, T: RateMatrix> Iterator for ChangeOrder<'a, T> {
//     type Item = &'a Node;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.tree.changes.is_empty() {
//             return None
//         } else if self.current_vec.is_empty() {
//             let k = *self.tree.changes.keys().max().unwrap();
//             self.current_vec = self.tree.changes.remove(&k).unwrap();
//             let x = self.current_vec.pop().unwrap();
//             let n: &'a Node = self.tree.get_node(x)?;
//             return Some(n)
//         } else {
//             let x = self.current_vec.pop().unwrap();
//             let n: Self::Item = self.tree.get_node(x)?;
//             return Some(n)
//         };
//     }
// }

// impl<'a> Tree {
//     pub fn changeiter(&'a self) -> ChangeOrder {
//         let max_depth = *self.changes.keys().max().unwrap();
//         let mut flvec: Vec<Vec<usize>> = vec![Vec::new(); max_depth];

//         for i in (0..=max_depth).rev() {
//             let nodes: Option<Vec<usize>> = self.changes.remove(&i);
//             if nodes.is_none() {
//                 continue;
//             }
//             for node in nodes.unwrap().iter() {
//                 let p = self.get_parent(*node);
//                 let pd = match i {
//                     0 => 0,
//                     _ => i - 1,
//                 };
//                 flvec[i].push(*node);
//                 if p.is_some() {
//                     flvec[pd].push(p.unwrap().index);
//                 }
//             }
//         }

//         for v in flvec {
//             v.dedup();
//         }

//         let out: Vec<usize> = flvec.into_iter().flatten().collect();
//         ChangeOrder { nodevec: Some(out) }
//     }
// }
