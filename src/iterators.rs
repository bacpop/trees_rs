use crate::topology::NodeTuple;
use crate::topology::Topology;
use std::collections::HashMap;

//////////////////////////////////
// Post-order Topology Iterator //
//////////////////////////////////
pub struct PostOrderIter<'a> {
    start_flag: bool,
    current_node: &'a NodeTuple,
    topology: &'a Topology,
    end_node: &'a NodeTuple,
}

// Enum for handedness of child nodes
#[derive(Debug)]
pub enum Handedness {
    Left,
    Right,
}

impl<'a> Iterator for PostOrderIter<'a> {
    type Item = &'a NodeTuple;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start_flag {
            self.current_node = self.topology.most_left_child(self.current_node);
            self.start_flag = false;
        } else {
            if self.current_node.eq(self.end_node)  {
                return None
            }

            match self.topology.get_handedness(self.current_node) {
                Handedness::Left => {
                    let r_child = self.topology.swap_to_right_child(self.current_node).unwrap();
                    self.current_node = self.topology.most_left_child(r_child);
                },
                Handedness::Right => {
                    self.current_node = &self.topology.nodes[self.current_node.get_parent().unwrap()];
                },
            }
        }

        Some(self.current_node)
    }
}

impl<'a> Topology {
    pub fn postorder(&'a self, node: &'a NodeTuple) -> PostOrderIter {
        PostOrderIter{
            current_node: node,
            start_flag: true,
            topology: self,
            end_node: node,
        }
    }

    pub fn postorder_notips(&'a self, node: &'a NodeTuple) -> impl Iterator<Item = &'a NodeTuple> {
        self.postorder(node).filter(|node| node.get_lchild().is_some() && node.get_rchild().is_some())
    }

    pub fn most_left_child(&'a self, node: &'a NodeTuple) -> &NodeTuple {
        let mut current_node = node;
        let mut current_left_child = self.get_lchild(current_node);

        while current_left_child.is_some() {
            current_node = current_left_child.unwrap();
            current_left_child = self.get_lchild(current_node);
        }
        current_node
    }

    pub fn get_handedness(&self, node: &NodeTuple) -> Handedness {
        
        let parent = self.get_parent(node);
        let parent_left_child: Option<usize>;

        if let Some(parent_node) = parent {
            parent_left_child = parent_node.get_lchild();
        } else {
            parent_left_child = self.get_root().get_lchild();
        };

        if node.get_id().eq(&parent_left_child.unwrap()) {
            return Handedness::Left;
        } else {
            return Handedness::Right;
        }
    }

    pub fn swap_to_right_child(&self, node: &NodeTuple) -> Option<&NodeTuple> {
        let parent = self.get_parent(node);
        if parent.is_some() {
            return self.get_rchild(parent.unwrap());
        } else {
            return self.get_rchild(self.get_root());
        }
    }
}

////////////////////////////////////
// Topology Iterator over changes //
////////////////////////////////////

pub struct ChangeIter<'a> {
    topology: &'a Topology,
    hmap: HashMap<usize, Vec<usize>>,
    current_vec: Vec<usize>,
}

impl<'a> Iterator for ChangeIter<'a> {
    type Item = &'a NodeTuple;
    fn next(&mut self) -> Option<Self::Item> {
        if self.hmap.is_empty() {
            return None;
        }

        if self.current_vec.is_empty() {
            let max_depth: usize = *self.hmap.keys().max().unwrap();
            self.current_vec = self.hmap.remove(&max_depth).unwrap();
            self.current_vec.sort();
            self.current_vec.dedup();
        }

        let current_id = self.current_vec.pop().unwrap();
        let current_node = &self.topology.nodes[current_id];

        // Add parent to hashmap
        if let Some(parent_index) = current_node.get_parent() {
            let parent_depth = self.topology.nodes[parent_index].get_depth();

            match self.hmap.get(&parent_depth) {
                None => {
                    self.hmap.insert(parent_depth, vec![parent_index]);
                },
                Some(_) => {
                    self.hmap.get_mut(&parent_depth).unwrap().push(parent_index);
                },
            };
        }


        return Some(current_node);
    }
}

impl<'a> Topology {
    pub fn changes_iter(&'a self, indices: Vec<usize>) -> ChangeIter {

        let mut hmap: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in indices {
            let d = self.nodes[i].get_depth();
            match hmap.get(&d) {
                None => {
                    hmap.insert(d, vec![i]);
                },
                Some(_) => {
                    hmap.get_mut(&d).unwrap().push(i);
                },
            }
        }

        ChangeIter{
            topology: self,
            hmap,
            current_vec: Vec::new(),
        }
    }

    pub fn changes_iter_notips(&'a self, indices: Vec<usize>) -> impl Iterator<Item = &'a NodeTuple> {
        self.changes_iter(indices).filter(|node| node.get_lchild().is_some() && node.get_rchild().is_some())
    }
}