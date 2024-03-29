use std::f64::NEG_INFINITY;
use std::thread::current;
use crate::combine_lists;
use crate::Mutation;
use crate::Tree;

impl Tree {
    // Goes through all nodes that have changed and updates genetic likelihood
    // Used after update_tree()
    pub fn update_likelihood(&mut self, rate_matrix: &na::Matrix4<f64>) {

        if self.changes.is_empty() {
            return
        }

        let max_depth: usize = *self.changes.keys().max().unwrap();

        for current_depth in (0..=max_depth).rev() {
            
            let mut nodes: Vec<usize> = self.changes.remove(&current_depth).unwrap();
            nodes.sort();
            nodes.dedup();

            let parent_depth: usize = match current_depth {
                0 => 0,
                _ => current_depth - 1,
            };

            // Traverse all nodes at current_depth
            for node in nodes {

                self.update_node_likelihood(node, rate_matrix);

                if current_depth > 0 {
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
            self.update_node_likelihood(node, rate_matrix);
        }
    }

    // Updates the genetic likelihood at a given node
    pub fn update_node_likelihood(&mut self, index: usize, rate_matrix: &na::Matrix4<f64>) {
        if let (Some(ch1), Some(ch2)) = self.get_node(index).unwrap().children {
            let branchlengths = (self.get_branchlength(ch1), self.get_branchlength(ch2));

            let seq1 = self.mutation_lists.get(ch1).unwrap();
            let seq2 = self.mutation_lists.get(ch2).unwrap();

            self.mutation_lists[index] = combine_lists(seq1, seq2, branchlengths, rate_matrix);
        }
    }

    pub fn get_tree_likelihood(&self) -> f64 {
        self.mutation_lists
            .get(self.get_root().unwrap().index)
            .unwrap()
            .iter()
            .fold(0.0, |acc, muta| {
                acc + (f64::exp(muta.0) * 0.25 + f64::exp(muta.1) * 0.25 + f64::exp(muta.2) * 0.25 + f64::exp(muta.3) * 0.25).ln()
            })
    }

}

impl Mutation {

    pub fn sum(self, r: Mutation) -> Mutation {
        Mutation(
            self.0 + r.0,
            self.1 + r.1,
            self.2 + r.2,
            self.3 + r.3,
        )
    }

    pub fn child_log_likelihood(self, prob_matrix: &na::Matrix4<f64>) -> Self {
        let lnx = vec![self.0, self.1, self.2, self.3];
        let mut x: Vec<f64> = Vec::new();

        for i in 0..=3 {
            x.push(logse(prob_matrix.row(i).iter().zip(&lnx).map(|(a, b)| a.ln() + b).collect()));
        }
        Mutation(*x.get(0).unwrap(), *x.get(1).unwrap(), *x.get(2).unwrap(), *x.get(3).unwrap())
    }
}

pub fn logse(x: Vec<f64>) -> f64 {
        let xstar = x.iter().max_by(|x, y| x.total_cmp(y)).unwrap();
        xstar + x.iter().fold(0.0,|acc, el| acc + f64::exp(el - xstar)).ln()
}