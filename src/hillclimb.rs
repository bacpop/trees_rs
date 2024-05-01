use rand::Rng;
use crate::Tree;

// Makes random moves +/- 1 moves on the integer vector (v) for a given number of elements (n)
pub fn peturb_vector(v: &[usize], n: usize) -> Vec<usize> {
    let mut vout = v.to_vec();
    let mut rng = rand::thread_rng();
    let ind_rng = rand::thread_rng();
    let distr = rand::distributions::Bernoulli::new(0.5).unwrap();
    let ind_distr = rand::distributions::Uniform::new(0, v.len());

    let mut inds: Vec<usize> = ind_rng.sample_iter(ind_distr).take(n).collect();
    inds.sort();

    for ind in inds {
        match rng.sample(distr) {
            true => {vout[ind] += 1;},
            false => {vout[ind] -= 1;},
        };  
        if ind.eq(&0) || vout[ind].lt(&0) {
           vout[ind] = 0;
        } else if vout[ind].gt(&(2 * (ind - 1))) {
           vout[ind] = 2 * (ind - 1);
        }
    };

    vout
}

impl Tree {
    // Hill climbing optimisation algorithm
    pub fn hillclimb(&mut self, q: &na::Matrix4<f64>, iterations: usize) {

        let mut working_tree: Tree = Tree {
            tree_vec: self.tree_vec.clone(),
            nodes: self.nodes.clone(),
            max_depth: self.max_depth,
            label_dictionary: self.label_dictionary.clone(),
            changes: self.changes.clone(),
            mutation_lists: self.mutation_lists.clone()
        };

        let mut candidate_vec: Vec<usize>;
        let mut best_vec: Option<Vec<usize>> = None;
        let mut best_likelihood: f64 = working_tree.get_tree_likelihood();
        for k in 0..=iterations {
            
            println!("Optimisation step {} out of {}", k, iterations);
            // println!("Old vector {:?}", self.tree_vec);
            println!("Current best likelihood: {}", best_likelihood);

            candidate_vec = peturb_vector(&self.tree_vec, self.tree_vec.len());
            working_tree.update(&candidate_vec);
            working_tree.update_likelihood(q);
            let new_likelihood = working_tree.get_tree_likelihood();

            // println!("New vector {:?}", candidate_vec);
            println!("Candidate likelihood {}", new_likelihood);

            if new_likelihood > best_likelihood {
                println!("Climbing hill!");
                best_vec = Some(working_tree.tree_vec.clone());
                best_likelihood = new_likelihood;
            }
        };

        if let Some(vec) = best_vec {
            self.update(&vec);
            self.update_likelihood(q);
        }
        
    }

}