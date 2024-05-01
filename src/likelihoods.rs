use crate::mutation::{Mutation, to_mutation};
use crate::Tree;
// Default base frequencies
const BF_DEFAULT: [f64; 4] = [0.25, 0.25, 0.25, 0.25];

// Calculates the likelihood at a base given the bases at each child and probability matrices
pub fn base_likelihood(mut1: &Mutation, mut2: &Mutation, p1: &na::Matrix4<f64>, p2: &na::Matrix4<f64>) -> Mutation {
    let v1: Vec<f64> = mut1.to_vector();
    let v2: Vec<f64> = mut2.to_vector();
    let mut x1: Vec<f64> = Vec::new();
    let mut x2: Vec<f64> = Vec::new();

    for i in 0..=3 {
        x1.push(logse(p1.row(i).iter().zip(&v1).map(|(a, b)| a.ln() + b).collect()));
        x2.push(logse(p2.row(i).iter().zip(&v2).map(|(a, b)| a.ln() + b).collect()));
    };

    to_mutation(x1).add(to_mutation(x2))
}

// Calculates the likelihood for a Node by calculating the likelihood at each base
pub fn calculate_likelihood(
    seq1: &[Mutation],
    seq2: &[Mutation],
    branchlengths: (f64, f64),
    rate_matrix: &na::Matrix4<f64>,
) -> Vec<Mutation> {

    // Probability matrices
    let p1 = na::Matrix::exp(&(rate_matrix * branchlengths.0));
    let p2 = na::Matrix::exp(&(rate_matrix * branchlengths.1));

    let out: Vec<Mutation> = seq1.iter()
    .zip(seq2.iter())
    .map(|(b1, b2)| base_likelihood(b1, b2, &p1, &p2))
    .collect();

    out
}

// LogSumExp function
pub fn logse(x: Vec<f64>) -> f64 {
    let xstar = x.iter().max_by(|x, y| x.total_cmp(y)).unwrap();
    xstar + x.iter().fold(0.0,|acc, el| acc + f64::exp(el - xstar)).ln()
}

// LogSumExp function that includes base frequency values for final likelihood calculation
pub fn base_freq_logse(muta: &Mutation, bf: [f64; 4]) -> f64 {
    (f64::exp(muta.0) * bf[0] + f64::exp(muta.1) * bf[1] + f64::exp(muta.2) * bf[2] + f64::exp(muta.3) * bf[3]).ln()
}

impl Tree {

    // Updates the genetic likelihood at a given node
    pub fn update_node_likelihood(&mut self, index: usize, rate_matrix: &na::Matrix4<f64>) {
        if let (Some(ch1), Some(ch2)) = self.get_node(index).unwrap().children {
            let branchlengths = (self.get_branchlength(ch1), self.get_branchlength(ch2));
    
            let seq1 = self.mutation_lists.get(ch1).unwrap();
            let seq2 = self.mutation_lists.get(ch2).unwrap();
    
            self.mutation_lists[index] = calculate_likelihood(seq1, seq2, branchlengths, rate_matrix);
        }
    }

    // Goes through all nodes that have changed and updates genetic likelihood
    // Used after tree.update()
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
    pub fn initialise_likelihood(&mut self, rate_matrix: &na::Matrix4<f64>) {
        let nodes: Vec<usize> = self
            .postorder_notips(self.get_root())
            .map(|n| n.index)
            .collect();

        for node in nodes {
            self.update_node_likelihood(node, rate_matrix);
        }
    }

    // Fetches likelihood value for a tree
    pub fn get_tree_likelihood(&self) -> f64 {
        self.mutation_lists
            .get(self.get_root().unwrap().index)
            .unwrap()
            .iter()
            .fold(0.0, |acc, muta| {
                // acc + (f64::exp(muta.0) * 0.25 + f64::exp(muta.1) * 0.25 + f64::exp(muta.2) * 0.25 + f64::exp(muta.3) * 0.25).ln()
                acc + base_freq_logse(muta, BF_DEFAULT)
            })
    }

}