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

    pub fn edit_node(&mut self, index: usize) -> Option<&mut Node> {
        self.nodes.get_mut(index)
    }

    pub fn edit_parent(&mut self, index: usize) -> Option<&mut Node> {
        match self.nodes.get(index).unwrap().parent {
            Some(i) => self.nodes.get_mut(i),
            None => None,
        }
    }

    pub fn read_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    pub fn read_parent(&self, index: usize) -> Option<&Node> {
        match self.nodes.get(index).unwrap().parent {
            Some(i) => self.nodes.get(i),
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
            self.edit_parent(index).unwrap().new_child(index);
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

        let cur_node = self.tree.read_node(self.c_index);
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


pub fn str2tree(st: String, name: String) -> Tree {
    let mut tree: Tree = Tree::new(name);
    let mut par_index: Option<usize> = None;
    let mut pin: usize = 0;

    for e in st.chars() {
        if e == ')' {
            par_index = match tree.read_node(pin) {
                None => None,
                Some(n) => n.parent,
            };
            pin = match par_index {
                None => 0,
                Some(x) => x,
            };
        } else if e != '(' {

        // } else {
            // Add node
            tree.add(String::from(e), par_index);
            pin = tree.nodes.len() - 1;
            par_index = Some(pin);
        }
    }

    tree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn treemake() {
        let ts = String::from("4(2(3)(1))(6(5))");
        let tree = str2tree(ts, String::from("Tree1"));
        
        assert_eq!(tree.read_node(0).unwrap().parent, None);
        assert_eq!(tree.read_node(4).unwrap().parent, Some(0));
        assert_eq!(tree.read_parent(1).unwrap().children, (Some(1), Some(4)));
    }
}