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
        RootIter{current: node, next: node, tree: self, end: false}
    }

    pub fn leftiter(&'a self, node: Option<&'a Node>) -> LeftIter {
        LeftIter{current: node, next: node, tree:self, returns: vec![]}
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
    current: Option<&'a Node>,
    next: Option<&'a Node>,
    tree: &'a Tree,
    end: bool,
}

// Iterates from a Node up to the root
impl<'a> Iterator for RootIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Self::Item>;

        if self.end {return None};

        match self.current.unwrap().parent {
            None => {
                output = self.tree.get_root();
                self.end = true;
            },
            Some(i) => {
                output = self.current;
                self.next = self.tree.get_node(i);
            },
        };

        self.current = self.next;

        output
    }
}

#[derive(Debug)]
pub struct LeftIter<'a> {
    current: Option<&'a Node>,
    next: Option<&'a Node>,
    returns: Vec<Option<&'a Node>>,
    tree: &'a Tree,
}

// Iterates downwards from a given node, visiting
// all left children first before returning to closest
// right child
impl<'a> Iterator for LeftIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Self::Item> = self.current; 

        if self.current.is_none() {
            return output;
        }

        match self.current.unwrap().children {
        (Some(a), None) => {
            self.next = self.tree.get_node(a);
        },

        (Some(a), Some(b)) => {
           self.next = self.tree.get_node(a);
           self.returns.push(self.tree.get_node(b));
        },
            
        (None, None) => {
            self.next = match self.returns.pop() {
                None => None,
                Some(node) => node,
            };  
        },

        _ => {panic!("Iterator has found a node with only a right child")},
        };

        self.current = self.next;
        
        output
    }
}
