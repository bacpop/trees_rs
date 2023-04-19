use std::fmt;

#[derive(Debug)]
pub struct Node {
    pub sample_name: String,
    pub parent: Option<usize>,
    pub children: (Option<usize>, Option<usize>),
    pub tip: bool,
    pub index: usize,
}


impl Node {
    pub fn new(sample_name:String, parent: Option<usize>, 
        children: (Option<usize>, Option<usize>), index: usize) -> Node {

        Node {sample_name, children, parent, index,
            tip: matches!(children, (None, None)),
            }
    }

    pub fn new_child(&mut self, new_child: usize) {

        self.children = match self.children {
            (None, None) => (Some(new_child), None),
            (Some(x), None) => (Some(x), Some(new_child)),
            (None, Some(y)) => (Some(new_child), Some(y)),
            (Some(_), Some(_)) => panic!("Trying to add new child to node with 2 children already"),
        };

        self.tip = false;
        
    }

    pub fn remove_child(&mut self, child: usize) {
        self.children = match self.children {
            (Some(a), Some(b)) if a == child => (Some(b), None),
            (Some(a), Some(b)) if b == child => (Some(a), None),
            (Some(a), None) if a == child => (None, None),
            (None, Some(_b)) => {
                panic!("Trying to remove child from parent with only a right child!")},
            _ => panic!("Trying to remove child that does not exist in parent"),
        };
    }
    // pub fn left_child(&self) -> Option<usize> {
    //     match self.children {
    //         (Some(x), _) => Some(x),
    //         (None, _) => None
    //     }
    // }

    // pub fn right_child(&self) -> Option<usize> {
    //     match self.children {
    //         (_, Some(x)) => Some(x),
    //         (_, None) => None
    //     }
    // }

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
        
        write!(f, "Sample Name: {}, sample index: {}, parent index: {}, child indices: {},{}", 
        self.sample_name, self.index, par, ch.0, ch.1)
    }
}