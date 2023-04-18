use crate::node::Node;

#[derive(Debug)]
pub struct Tree {
    pub tree_name: String,
    pub nodes: Vec<Node>,
}

impl<'a> Tree {
    pub fn new(tree_name: String) -> Tree {
        Tree {tree_name,
        nodes: Vec::new()}
    }

    pub fn get_root(&self) -> Option<&Node> {
        self.get_node(0)
    }

    pub fn iter(&'a self, node: Option<&'a Node>) -> RootIter {
        RootIter{cnode: node, nnode: node, tree: self, end: false}
    }

    pub fn leftiter(&'a self, node: Option<&'a Node>) -> LeftIter {
        LeftIter{cnode: node, nnode: node, tree:self, ret_vec: vec![]}
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

    // pub fn getn_parent(&self, node: Option<&Node>) -> Option<&Node> {
    //     match node.unwrap().parent {
    //         None => None,
    //         Some(i) => self.get_node(i),
    //     }
    // }

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
pub struct RootIter<'a> {
    cnode: Option<&'a Node>,
    nnode: Option<&'a Node>,
    tree: &'a Tree,
    end: bool,
}

// Iterates from a Node up to the root
impl<'a> Iterator for RootIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let out: Option<Self::Item>;

        // Check current node parent
        match self.cnode.unwrap().parent {
            // Parent doesn't exist
            None => {
                if !self.end {
                    // Return root, then trigger Iterator end
                    out = self.tree.get_node(0);
                    self.end = true;
                } else {
                    // End Iterator
                    out = None;
                }
            },
            // Parent exists with index i, return
            // current node then get next node
            Some(i) => {
                out = self.cnode;
                self.nnode = self.tree.get_node(i);
            },
        };
        // Update current node to next node
        self.cnode = self.nnode;

        out
    }
}

#[derive(Debug)]
pub struct LeftIter<'a> {
    cnode: Option<&'a Node>,
    nnode: Option<&'a Node>,
    ret_vec: Vec<Option<&'a Node>>,
    tree: &'a Tree,
}
// Iterates downwards from a given node, visiting
// all left children first before returning to closest
// right child
impl<'a> Iterator for LeftIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let out: Option<Self::Item> = self.cnode; 

        // Can skip all this if current Node is None
        if self.cnode.is_some() {

        match self.cnode.unwrap().children {
            // Current Node has left child only
            (Some(a), None) => {
                // Get next Node from left child
                self.nnode = self.tree.get_node(a);
            },
            // Current Node has left and right child
            (Some(a), Some(b)) => {
                // Get next Node from left child
                self.nnode = self.tree.get_node(a);
                // Add right child to vector to return to
                self.ret_vec.push(self.tree.get_node(b));
            },
            // Current Node has no children
            (None, None) => {
                // Look at latest entry in return vector
                self.nnode = match self.ret_vec.pop() {
                    // If None => No nodes to return to, set next Node to
                    // None which will end iterator next go
                    None => None,
                    // If Some(node), next node is this node
                    Some(node) => node,
                };  
            },
            // This shouldn't happen since all children are allocated to the left
            // first so Nodes with 1 child have left children only.
            _ => {panic!("Iterator has found a node with only a right child")},
        };

        self.cnode = self.nnode;
        }
        out
    }
}
