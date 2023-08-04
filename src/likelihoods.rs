use std::thread::current;

use crate::Tree;

impl Tree {
    pub fn update_likelihood(&mut self, rate_matrix: &na::Matrix4<f64>) {
        
        if !self.changehm.is_empty() {

        let max_depth: usize = *self.changehm.keys().max().unwrap();

        for current_depth in (0..=max_depth).rev() {
            
            let mut nodes: Vec<usize> = self.changehm.remove(&current_depth).unwrap();
            nodes.sort();
            nodes.dedup();

            let parent_depth: usize = if current_depth == 0 {0} else {current_depth - 1};

            // Traverse all nodes at current_depth
            for node in nodes {
                // Line here to update this node
                // Something like:
                // tr.update_likelihood(node, ll.likelihood_lists, rate_matrix);

                // Put parent into HashMap so that they are updated
                let parent: usize = self.get_parent(node).unwrap().index;

                match self.changehm.get(&parent_depth) {
                    None => {
                        self.changehm.insert(parent_depth, vec![parent]);
                    }
                    Some(_) => {
                        self.changehm.get_mut(&parent_depth).unwrap().push(parent);
                    }
                }
            }
            
        }

    }
    }
}