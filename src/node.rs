use crate::gen_list::Mutation;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Node {
    pub parent: Option<usize>,
    pub children: (Option<usize>, Option<usize>),
    pub tip: bool,
    pub index: usize,
    pub depth: usize,
    pub branch_length: f64,
}

impl Node {
    pub fn new(
        parent: Option<usize>,
        children: (Option<usize>, Option<usize>),
        index: usize,
        depth: usize,
        branch_length: f64,
    ) -> Node {
        Node {
            children,
            parent,
            index,
            depth,
            tip: matches!(children, (None, None)),
            branch_length,
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
                panic!("Trying to remove child from parent with only a right child!")
            }
            _ => panic!("Trying to remove child that does not exist in parent"),
        };
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

        write!(
            f,
            "Sample index: {}, parent index: {}, child indices: {},{}",
            self.index, par, ch.0, ch.1
        )
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            parent: None,
            children: (None, None),
            tip: false,
            index: 0,
            depth: 0,
            branch_length: 0.000001,
        }
    }
}
