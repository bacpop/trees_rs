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

        let mut candidate_vec: Vec<usize> = Vec::with_capacity(self.tree_vec.len());
        let mut best_vec: Vec<usize> = self.tree_vec.clone();
        let mut best_likelihood: f64 = self.get_tree_likelihood();
        let mut new_likelihood: f64;

        for k in 0..=iterations {
            println!("Optimisation step {} out of {}", k, iterations);

            candidate_vec = peturb_vector(&best_vec, self.tree_vec.len());
            self.update(&candidate_vec);
            self.update_likelihood(q);
            new_likelihood = self.get_tree_likelihood();
            println!("Candidate likelihood: {} \n Current likelihood: {}", new_likelihood, best_likelihood);

            if new_likelihood > best_likelihood {
                println!("Climbing hill!");
                best_vec = candidate_vec;
                best_likelihood = new_likelihood;
            }
        };

        self.update(&best_vec);
        self.update_likelihood(q);
        
    }

}