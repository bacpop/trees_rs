use std::fmt;

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

    pub fn left_child(&self) -> Option<usize> {
        match self.children {
            (Some(x), _) => Some(x),
            (None, _) => None
        }
    }

    pub fn right_child(&self) -> Option<usize> {
        match self.children {
            (_, Some(x)) => Some(x),
            (_, None) => None
        }
    }

}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let par = self.parent.unwrap_or(0);
        let ch: (String, String) = match self.children {
            (None, None) => (String::from("None"), String::from("None")),
            (None, Some(x)) => (String::from("None"), x.to_string()),
            (Some(y), None) => (y.to_string(), String::from("None")),
            (Some(x), Some(y)) => (x.to_string(), y.to_string()),
        };
        
        write!(f, "Sample Name: {}, parent index: {}, child indices: {},{}", 
        self.sample_name, par, ch.0, ch.1)
    }
}