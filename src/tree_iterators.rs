use clap::builder::NonEmptyStringValueParser;
use ndarray::AssignElem;

use crate::node::Node;
use crate::tree::Tree;

//////////////////////////////
// Post-order Tree iterator //
//////////////////////////////

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

// Tree methods used in post-order traversal
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
}

// Changes iterator
// #[derive(Debug)]
// pub struct ChangeOrder {
//     nodevec: Option<Vec<usize>>,
// }

// impl Iterator for ChangeOrder {
//     type Item = &'a Node;
// Will changes be faster and easier using
// Vec<Option<Vec<usize>>> ?
// fn next(&mut self) -> Option<Self::Item> {
//     if self.tree.changes.is_empty() {
//         return None;
//     }

//     while !self.tree.changes.contains_key(&self.depth) {
//         self.depth -= 1;
//     }

//     let x = self.tree.changes.remove(&self.depth).unwrap();
//     let ni = x.pop();
//     self.tree.changes.insert(self.depth, x);

//     next_node
// }
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
