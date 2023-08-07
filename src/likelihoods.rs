use crate::combine_lists;
use crate::Tree;

impl Tree {
    // Goes through all nodes that have changed and updates genetic likelihood
    // Used after update_tree()
    pub fn update_likelihood(&mut self, rate_matrix: &na::Matrix4<f64>) {
        if !self.changes.is_empty() {
            let max_depth: usize = *self.changes.keys().max().unwrap();

            for current_depth in (0..=max_depth).rev() {
                let mut nodes: Vec<usize> = self.changes.remove(&current_depth).unwrap();
                nodes.sort();
                nodes.dedup();

                let parent_depth: usize = if current_depth == 0 {
                    0
                } else {
                    current_depth - 1
                };

                // Traverse all nodes at current_depth
                for node in nodes {
                    // Line here to update this node
                    // Something like:
                    self.update_likelihood_node(node, rate_matrix);

                    // Put parent into HashMap so that they are updated
                    let parent: usize = self.get_parent(node).unwrap().index;

                    match self.changes.get(&parent_depth) {
                        None => {
                            self.changes.insert(parent_depth, vec![parent]);
                        }
                        Some(_) => {
                            self.changes.get_mut(&parent_depth).unwrap().push(parent);
                        }
                    }
                }
            }
        }
    }

    // Traverses tree below given node (except leaves), updating likelihood
    // Used after initial tree constructions to fill in likelihood at all internal nodes
    pub fn update_likelihood_postorder(&mut self, rate_matrix: &na::Matrix4<f64>) {
        let nodes: Vec<usize> = self
            .postorder_notips(self.get_root())
            .map(|n| n.index)
            .collect();

        for node in nodes {
            self.update_likelihood_node(node, rate_matrix);
        }
    }

    // Updates the genetic likelihood at a given node
    pub fn update_likelihood_node(&mut self, index: usize, rate_matrix: &na::Matrix4<f64>) {
        if let (Some(ch1), Some(ch2)) = self.get_node(index).unwrap().children {
            let branchlengths = (self.get_branchlength(ch1), self.get_branchlength(ch2));

            let seq1 = self.mutation_lists.get(ch1);
            let seq2 = self.mutation_lists.get(ch2);

            self.mutation_lists[index] = combine_lists(seq1, seq2, branchlengths, rate_matrix);
        }
    }

    pub fn get_likelihood(self) -> f64 {
        self.mutation_lists.get(self.get_root().unwrap().index)
            .unwrap()
            .iter()
            .fold(0.0, |acc, muta| 
                    (acc + muta.1 * 0.25 + muta.2 * 0.25 + muta.3 * 0.25 + muta.4 * 0.25))
    }
}
