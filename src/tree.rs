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
        RootIter{current_node: node, next_node: node, 
            tree: self, end_flag: false}
    }

    pub fn preorder(&'a self, node: Option<&'a Node>) -> Preorder {
        Preorder{current_node: node, next_node: node, 
            tree:self, return_nodes: vec![]}
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
    current_node: Option<&'a Node>,
    next_node: Option<&'a Node>,
    tree: &'a Tree,
    end_flag: bool,
}

// Traverses from a given Node up to the root of the tree
impl<'a> Iterator for RootIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Self::Item>;

        if self.end_flag {return None};

        match self.current_node.unwrap().parent {
            None => {
                output = self.tree.get_root();
                self.end_flag = true;
            },
            Some(i) => {
                output = self.current_node;
                self.next_node = self.tree.get_node(i);
            },
        };

        self.current_node = self.next_node;

        output
    }
}

#[derive(Debug)]
pub struct Preorder<'a> {
    current_node: Option<&'a Node>,
    next_node: Option<&'a Node>,
    return_nodes: Vec<Option<&'a Node>>,
    tree: &'a Tree,
}

// Traverses tree in preorder starting from specified node
impl<'a> Iterator for Preorder<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let output: Option<Self::Item> = self.current_node; 

        if self.current_node.is_none() {
            return output;
        }

        match self.current_node.unwrap().children {
        (Some(a), None) => {
            self.next_node = self.tree.get_node(a);
        },

        (Some(a), Some(b)) => {
           self.next_node = self.tree.get_node(a);
           self.return_nodes.push(self.tree.get_node(b));
        },
            
        (None, None) => {
            self.next_node = match self.return_nodes.pop() {
                None => None,
                Some(node) => node,
            };  
        },

        _ => {panic!("Iterator has found a node with only a right child")},
        };

        self.current_node = self.next_node;
        
        output
    }
}
