use crate::node::Node;

#[derive(Debug)]
pub struct Tree {
    pub tree_vec: Vec<usize>,
    pub nodes: Vec<Node>,
}

impl<'a> Tree {
    pub fn new(tree_vec: Vec<usize>) -> Tree {
        let k = tree_vec.len();
        Tree {tree_vec,
        nodes: vec![Node::default(); 2 * k + 1]}
    }

    pub fn get_root(&self) -> Option<&Node> {
        self.nodes.iter().find(|node| node.parent.is_none())
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

    // Returns vector of nodes in tree that are tips
    pub fn get_tips(&self) -> Vec<&Node> {
        self.nodes
        .iter()
        .filter(|n| n.tip == true)
        .collect()
    }

    // Depth of given node in tree
    pub fn node_depth(&self, node: Option<&Node>) -> usize {
        self
        .iter(node)
        .fold(0, |acc, _node| acc + 1)
    }

    // Find maximum depth across all tips
    pub fn max_treedepth(&self) -> usize {
        let depths_vec: Vec<usize> = self
            .get_tips()
            .iter()
            .map(|t| self.node_depth(Some(t)))
            .collect();
            
        match depths_vec.iter().max() {
            None => 0,
            Some(i) => *i,
        }
    }

    pub fn add(&mut self, index: usize, parent: Option<usize>){

        // if self.get_root().is_some() && parent.is_none(){
        //     panic!("Trying to assign a second root with parent = None");
        // }

        // let index: usize = self.nodes.len();

        // self.nodes.push(Node::new(parent, (None, None), index));
        self.nodes[index] = Node::new(parent, (None, None), index);

        if parent.is_some(){
            self.mut_parent(index).unwrap().new_child(index)
        };

    }

    pub fn relocate(&mut self, node_index: usize, new_parent_index: usize) {

        if self.get_node(node_index).is_none() {
            panic!("Node to move does not exist");
        }

        if self.get_node(new_parent_index).is_none() {
            panic!("New parent does not exist");
        }

        if self.get_parent(node_index).is_none() {
            panic!("Cannot move root node")
        }

        self.mut_parent(node_index).unwrap().remove_child(node_index);
        self.mut_node(node_index).unwrap().parent = Some(new_parent_index);
        self.mut_node(new_parent_index).unwrap().new_child(node_index);

    }

    pub fn most_left_child(&'a self, node: Option<&'a Node>) -> Option<&Node> {
        let mut cur_node = node;
        let mut cur_left_child = cur_node.unwrap().children.0;

        while cur_left_child.is_some() {
            cur_node = self.get_node(cur_left_child.unwrap());
            cur_left_child = cur_node.unwrap().children.0;
        }
        // println!("current node: {:?}", cur_node);
        cur_node
    }

}
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

        if self.current_node.is_none() {return output;}

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

// pub struct PostOrder<'a> {
//     tree: &'a Tree,
// }

// impl<'a> Iterator for PostOrder<'a> {
//     type Item = &'a Node;

//     fn next(&mut self) -> Option<Self::Item> {
        
//     }
// }