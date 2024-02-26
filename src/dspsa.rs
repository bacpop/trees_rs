use rand::Rng;
use crate::Tree;

pub fn phi(v: &[f64]) -> Vec<f64> {
    v.iter().enumerate().map(|(i, value)| {
        if i == 0 || value.lt(&0.0) {
            0.0
        } else if value.gt(&(i as f64)) {
            (i as f64) - 0.001
        } else {
            *value
        }
    }).collect()
}

pub fn piv(v: &[f64]) -> Vec<f64> {
    let mut pivec: Vec<f64> = phi(v).iter().map(|el| el.floor() + 0.5).collect();
    pivec[0] = 0.0;
    pivec
}

pub fn peturbation_vec(n: usize) -> Vec<f64> {
    let rng = rand::thread_rng();
    let distr = rand::distributions::Bernoulli::new(0.5).unwrap();
    let mut delta: Vec<f64> = rng.sample_iter(distr).take(n).map(|el| match el {
        true => 1.0,
        false => -1.0,
    }).collect();
    delta[0] = 0.0;
    delta
}

pub fn theta_change(pivec: &Vec<f64>, delta: &Vec<f64>, plus: bool) -> Vec<usize> {
    
    let zip = pivec.iter().zip(delta.iter());
    
    match plus {
        true => {
            zip
            .map(|(x, y)| (x + (y / 2.0)).round() as usize)
            .collect()
        },
        false => {
            zip
            .map(|(x, y)| (x - (y / 2.0)).round() as usize)
            .collect()
        }
    }
}

impl Tree {
    pub fn optimise(&mut self, q: &na::Matrix4<f64>, iterations: usize) {
        // Convert tree vector to Vec<f64>
        let mut theta: Vec<f64> = self.tree_vec.iter().map(|x| *x as f64).collect();
        println!("Current tree vector is: {:?}", theta);
        println!("Current likelihood is: {}", self.get_tree_likelihood());
        let n: usize = theta.len();

        // Tuning parameters for optimisation, will
        // eventually have defaults or be passed in
        let a: f64 = 2.0;
        let cap_a: f64 = 2.0;
        let alpha: f64 = 0.75;

        // Pre-allocate vectors
        let mut delta: Vec<f64> = Vec::with_capacity(n);
        let mut pivec: Vec<f64> = Vec::with_capacity(n);
        let mut thetaplus: Vec<usize> = Vec::with_capacity(n);
        let mut thetaminus: Vec<usize> = Vec::with_capacity(n);

        // Optimisation loop
        for k in 0..=iterations {
            println!("Optimisation step {} out of {}", k, iterations);
            // Generate peturbation vector
            delta = peturbation_vec(n);

            // Generate pi vector
            pivec = piv(&theta);

            // Calculate theta+ and theta-,
            // New tree vectors based on peturbation
            thetaplus = theta_change(&pivec, &delta, true);
            thetaminus = theta_change(&pivec, &delta, false);

            // Update tree and calculate likelihoods
            self.update_tree(Some(thetaplus), false);
            self.update_likelihood(&q);
            let lplus: f64 = self.get_tree_likelihood();

            self.update_tree(Some(thetaminus), false);
            self.update_likelihood(&q);
            let lminus: f64 = self.get_tree_likelihood();

            // Update theta based on likelihoods of theta+/-
            let ldiff = lplus - lminus;

            let ghat: Vec<f64> = delta.iter()
            .map(|el| if !el.eq(&0.0) {el * ldiff} else {0.0}).collect();

            let ak: f64 = a / (1.0 + cap_a + k as f64).powf(alpha);

            // Set new theta
            theta = theta.iter().zip(ghat.iter())
            .map(|(theta, g)| *theta - ak * g).collect();
        println!("New tree vector is: {:?}", theta);
        }

        // Update final tree after finishing optimisation
        println!("New tree vector is: {:?}", theta);
        let new_tree_vec: Vec<usize> = theta.iter().map(|x| *x as usize).collect();
        println!("New tree vector is: {:?}", new_tree_vec);
        self.update_tree(Some(new_tree_vec), false);
        self.update_likelihood(&q);
        println!("New tree likelihood is {}", self.get_tree_likelihood());
    }
}