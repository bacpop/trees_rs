use rand::Rng;

pub fn phi(v: Vec<f64>) -> Vec<f64> {
    v.iter().enumerate().map(|(i, value)| {
        if i == 0 {
            0.0
        } else if value < &0.0 {
            0.0
        } else if value > &(i as f64) {
            (i as f64) - 0.001
        } else {
            *value as f64
        }
    }).collect()
}

pub fn peturbation_vec(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Bernoulli::new(0.5).unwrap();
    let mut delta: Vec<f64> = rng.sample_iter(distr).take(20).map(|el| match el {
        true => 1.0,
        false => -1.0,
    }).collect();
    delta[0] = 0.0;
    delta
}