use crate::node::Node;

#[derive(Debug)]
pub struct Tree {
    pub tree_name: String,
    pub nodes: Vec<Node>,
}

impl Tree {
    pub fn new(tree_name: String) -> Tree {
        Tree {tree_name,
        nodes: Vec::new()}
    }

    pub fn iter(&self, index: usize) -> TreeIter {
        TreeIter{c_index: index, n_index: index, end: false,  tree: self}
    }

    pub fn leftiter(&self, index: usize) -> LeftIter {
        LeftIter{c_index: index, n_index: index, ret_vec: vec![], tree: self, start: true}
    }

    pub fn mut_node(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn mut_parent(&mut self, index: usize) -> Option<&mut Node> {
        match self.nodes.get(index).unwrap().parent {
            Some(i) => self.mut_node(i),
            None => None,
        }
    }

    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn get_parent(&self, index: usize) -> Option<&Node> {
        match self.nodes.get(index).unwrap().parent {
            Some(i) => self.get_node(i),
            None => None,
        }
    }

    pub fn add(&mut self, sample_name:String, parent: Option<usize>){

        // Add new node to vector of nodes in tree
        self.nodes.push(Node::new(sample_name, parent, (None, None)));

        // Update parent node to have children
        if parent.is_some(){
            // Work out node vector index of this new node
            let index = self.nodes.len() - 1;
        
            // Add new node index to children of parent
            self.mut_parent(index).unwrap().new_child(index);
        } else if parent.is_none() {
            // needs an error for when >1 node and you are trying to add another
            // with no parent
        };

    }

}

#[derive(Debug)]
pub struct TreeIter<'a> {
    c_index: usize,
    n_index: usize,
    end: bool,
    tree: &'a Tree,
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        
        // Basic structure:
        // current index = index
        // next index = index of parent of current index
        // output = current index
        // current index = next index

        let cur_node = self.tree.get_node(self.c_index);
        let out: Option<Self::Item>;
        
        match cur_node {
            // Current node doesn't exist
            None => {out = None},
            // Current node does exist
            Some(node) => {
                match node.parent {
                    // Parent is none - potentially end iterator
                    None => {
                        // The end boolean flags that we have reached the root 
                        // but permits one last element (the root) in the iterator
                        if self.end{
                            // end = true, end iterator
                            out = None;
                        }else {
                            // end = false, next node is root
                            // then set end = true
                            out = Some(0);
                            self.end = true;
                        } 
                    },
                    // Parent is Some(i), next index is i
                    // and output is current index
                    Some(i) => {
                        out = Some(self.c_index);
                        self.n_index = i;
                    }
                }
            }
        }
        // next index is now current index
        self.c_index = self.n_index;

        out
    }
}

pub struct LeftIter<'a> {
    start: bool,
    c_index: usize,
    n_index: usize,
    ret_vec: Vec<usize>,
    tree: &'a Tree,
}

impl<'a> Iterator for LeftIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {

        let out: Option<Self::Item>;

        let cur_children = self.tree.get_node(self.c_index).unwrap();

        if self.start {
            out = Some(self.c_index);
            self.start = false;
        } else {

        // check current node left child
        match cur_children.left_child() {
            None => {
                // No left child, next index is right child of
                // last node with a right child
                let new_parent = self.ret_vec.pop();

                match new_parent {
                    None => {out = None;}, // Nothing to return to, end iterator
                    Some(p) => {
                        // Next index is right child of node index p
                        out = self.tree.get_node(p).unwrap().right_child();
                        self.n_index = out.unwrap();
                    }
                }

            },
            Some(l) => {
                // There is a left child, next index is left child
                out = Some(l);
                self.n_index = l;
                // If current node has a right child, current index 
                // needs to be remembered
                if cur_children.right_child().is_some() {
                    self.ret_vec.push(self.c_index);
                }
            },
        };
        }

    // Current index is now next index
    self.c_index = self.n_index;
    
    out
    }
}