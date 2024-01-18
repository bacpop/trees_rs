use rand::Rng;

pub fn phi(v: &Vec<f64>) -> Vec<f64> {
    v.iter().enumerate().map(|(i, value)| {
        if i == 0 || value.lt(&0.0) {
            0.0
        } else if value.gt(&(i as f64)) {
            (i as f64) - 0.001
        } else {
            *value as f64
        }
    }).collect()
}

pub fn piv(v: &Vec<f64>) -> Vec<f64> {
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