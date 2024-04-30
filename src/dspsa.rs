use rand::Rng;
use crate::Tree;

pub fn hill_peturb(v: &[usize], n: usize) -> Vec<usize> {
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
        for k in 0..=iterations {
            
            println!("Optimisation step {} out of {}", k, iterations);
            // println!("Old vector {:?}", self.tree_vec);
            println!("Tree log likelihood: {}", self.get_tree_likelihood());

            candidate_vec = hill_peturb(&self.tree_vec, 28);
            working_tree.update_quad(&candidate_vec);
            working_tree.update_likelihood(q);

            // println!("New vector {:?}", candidate_vec);
            println!("New likelihood {}", working_tree.get_tree_likelihood());

            if working_tree.get_tree_likelihood() > self.get_tree_likelihood() {
                println!("Climbing hill!");
                self.update_quad(&working_tree.tree_vec);
                self.update_likelihood(q);
            }
        }
    }


    // pub fn theta_change(pivec: &Vec<f64>, delta: &Vec<f64>, plus: bool) -> Vec<usize> {

    //     let zip = pivec.iter().zip(delta.iter());
        
    //     match plus {
    //         true => {
    //             zip
    //             .map(|(x, y)| (x + (y / 2.0)).round() as usize)
    //             .collect()
    //         },
    //         false => {
    //             zip
    //             .map(|(x, y)| (x - (y / 2.0)).round() as usize)
    //             .collect()
    //         }
    //     }
    // }
// pub fn phi(v: &[f64]) -> Vec<f64> {
//     v.iter().enumerate().map(|(i, value)| {
//         if i == 0 || value.lt(&0.0) {
//             0.0
//         } else if value.gt(&((2 * (i - 1)) as f64)) {
//             ((2 * (i - 1)) as f64) - 0.000001
//         } else {
//             *value
//         }
//     }).collect()
// }

// pub fn piv(v: &[f64]) -> Vec<f64> {
//     let mut pivec: Vec<f64> = phi(v).iter().map(|el| el.floor() + 0.5).collect();
//     pivec[0] = 0.0;
//     pivec
// }

// pub fn peturbation_vec(n: usize) -> Vec<f64> {
//     let rng = rand::thread_rng();
//     let distr = rand::distributions::Bernoulli::new(0.5).unwrap();
//     let mut delta: Vec<f64> = rng.sample_iter(distr).take(n).map(|el| match el {
//         true => 1.0,
//         false => -1.0,
//     }).collect();
//     delta[0] = 0.0;
//     delta
// }

    // pub fn optimise(&mut self, q: &na::Matrix4<f64>, iterations: usize) {

    //     // Update likelihood if not done already
    //     // if self.get_tree_likelihood().eq(&0.0) {
    //     //     self.update_likelihood(&q);
    //     // }

    //     // Convert tree vector to Vec<f64>
    //     let mut theta: Vec<f64> = self.tree_vec.iter().map(|x| *x as f64).collect();
    //     println!("Current tree vector is: {:?}", self.tree_vec);
    //     println!("Current likelihood is: {}", self.get_tree_likelihood());
    //     let n: usize = theta.len();

    //     // Tuning parameters for optimisation, will
    //     // eventually have defaults or be passed in
    //     let a: f64 = 1.5;
    //     let cap_a: f64 = 1000.0;
    //     let alpha: f64 = 0.51;

    //     // Pre-allocate vectors
    //     let mut delta: Vec<f64> = Vec::with_capacity(n);
    //     let mut pivec: Vec<f64> = Vec::with_capacity(n);
    //     let mut thetaplus: Vec<usize> = Vec::with_capacity(n);
    //     let mut thetaminus: Vec<usize> = Vec::with_capacity(n);
    //     let mut ghat: Vec<f64> = Vec::with_capacity(n);
    //     let mut new_tree_vec: Vec<usize> = Vec::with_capacity(n);

    //     // Optimisation loop
    //     for k in 0..=iterations {
    //         println!("Optimisation step {} out of {}", k, iterations);
    //         println!("Negative tree log likelihood: {}", -self.get_tree_likelihood());
    //         // Generate peturbation vector
    //         delta = peturbation_vec(n);
    //         // println!("Peturbation vector: {:?}", delta);

    //         // Generate pi vector
    //         pivec = piv(&theta);
    //         // println!("Pi vector: {:?}", pivec);

    //         // Calculate theta+ and theta-,
    //         // New tree vectors based on peturbation
    //         thetaplus = theta_change(&pivec, &delta, true);
    //         thetaminus = theta_change(&pivec, &delta, false);
    //         // println!("theta+: {:?}", thetaplus);
    //         // println!("theta-: {:?}", thetaminus);

    //         // Update tree and calculate likelihoods
    //         self.update_quad(thetaplus);
    //         self.update_likelihood(&q);
    //         let lplus: f64 = -self.get_tree_likelihood();

    //         self.update_quad(thetaminus);
    //         self.update_likelihood(&q);
    //         let lminus: f64 = -self.get_tree_likelihood();

    //         // Update theta based on likelihoods of theta+/-
    //         let ldiff = lplus - lminus;

    //         println!("ll+ is {} and ll- is {}, ldiff is {}", lplus, lminus, ldiff);

    //         ghat = delta.iter().map(|delta| ldiff * (1.0 / delta)).collect();
    //         ghat[0] = 0.0;

    //         // println!("ghat is {:?}", ghat);

    //         let ak: f64 = a / (1.0 + cap_a + k as f64).powf(alpha);

    //         println!("ak is {}", ak);

    //         // Set new theta
    //         theta = theta.iter().zip(ghat.iter())
    //         .map(|(theta, g)| *theta - (ak * g)).collect();

    //         new_tree_vec = phi(&theta).iter().map(|x| x.round() as usize).collect();
    //         self.update_quad(new_tree_vec);
    //         self.update_likelihood(&q)
    //         // println!("New theta is: {:?}", theta);
    //     }

    //     // Update final tree after finishing optimisation
    //     // let new_tree_vec: Vec<usize> = phi(&theta).iter().map(|x| x.round() as usize).collect();
    //     // println!("New tree vector is: {:?}", new_tree_vec);
    //     // self.update_quad(new_tree_vec);
    //     // self.update_likelihood(&q);
    //     println!("New tree likelihood is {}", self.get_tree_likelihood());
    // }
}