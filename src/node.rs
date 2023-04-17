#[derive(Debug)]
pub struct Node {
    pub sample_name: String,
    pub parent: Option<usize>,
    pub children: (Option<usize>, Option<usize>),
    pub tip: bool,
}


impl Node {
    pub fn new(sample_name:String, parent: Option<usize>, children: (Option<usize>, Option<usize>)) -> Node {
        Node {sample_name, children, parent,
            tip: matches!(children, (None, None)),
            }
    }

    pub fn new_child(&mut self, new_child: usize) {
        // Work out where to put new child node index in children tuple
        self.children = match self.children {
            (None, None) => (Some(new_child), None),
            (Some(x), None) => (Some(x), Some(new_child)),
            (None, Some(y)) => (Some(new_child), Some(y)),
            (Some(_), Some(_)) => panic!("Trying to add new child to node with 2 children already"),
        };
        // Node with added child is no longer a tip
        self.tip = false;
        
    }

}