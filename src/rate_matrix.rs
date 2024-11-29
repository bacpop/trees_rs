use crate::topology::Topology;
use crate::{base_freq_logse, likelihood, matrix_exp, node_likelihood, slice_data, CandidateTopology, MoveFn, BF_DEFAULT};
use statrs::distribution::{Dirichlet};
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use ndarray::s;

pub trait RateMatrix: Copy {
    fn update_matrix(&mut self);

    fn update_params(&mut self, params: Vec<f64>);

    fn get_matrix(&self) -> na::Matrix4<f64>;

    fn get_params(&self) -> Vec<f64>;

    fn matrix_move(&self) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct GTR {
    matrix: na::Matrix4<f64>,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    p0: f64,
    p1: f64,
    p2: f64,
    p3: f64,
}

impl RateMatrix for GTR {

    fn get_params(&self) -> Vec<f64> {
        vec![self.a, self.b, self.c, self.d, self.e, self.f, self.p0, self.p1, self.p2, self.p3]
    }

    fn update_params(&mut self, params: Vec<f64>) {
        self.a = params[0];
        self.b = params[1];
        self.c = params[2];
        self.d = params[3];
        self.e = params[4];
        self.f = params[5];
        self.p0 = params[6];
        self.p1 = params[7];
        self.p2 = params[8];
        self.p3 = params[9];
        self.update_matrix();
    }

    fn get_matrix(&self) -> na::Matrix4<f64> {
        self.matrix
    }

    fn update_matrix(&mut self) {
        
        self.matrix = na::Matrix4::new(
                -(self.a * self.p1 + self.b * self.p2 + self.c * self.p3),
                self.a * self.p1,
                self.b * self.p2,
                self.c * self.p3,
                self.a * self.p0,
                -(self.a * self.p0 + self.d * self.p2 + self.e * self.p3),
                self.d * self.p2,
                self.e * self.p3,
                self.b * self.p0,
                self.d * self.p1,
                -(self.b * self.p0 + self.d * self.p1 + self.f * self.p3),
                self.f * self.p3,
                self.c * self.p0,
                self.e * self.p1,
                self.f * self.p2,
                -(self.c * self.p0 + self.e * self.p1 + self.f * self.p2));
    }

    fn matrix_move(&self) -> Self {
        let mut d1 = Dirichlet::new_with_param(1.0, 6).unwrap();
        let pars = d1.sample(&mut rand::thread_rng());
        
        let d2 = Dirichlet::new_with_param(1.0, 4).unwrap();
        let pars2 = d2.sample(&mut rand::thread_rng());

        let params: Vec<f64> = pars.iter().chain(pars2.iter()).map(|x| *x).collect();
        let mut new: Self = Self::default();
        new.update_params(params);
        new.update_matrix();
        new
    }
}

impl Default for GTR {
    fn default() -> Self{
        let mut out: GTR = GTR { matrix: na::Matrix4::identity(),
            a: 4.0 / 3.0,
            b: 4.0 / 3.0,
            c: 4.0 / 3.0,
            d: 4.0 / 3.0,
            e: 4.0 / 3.0,
            f: 4.0 / 3.0,
            p0: 0.25,
            p1: 0.25,
            p2: 0.25,
            p3: 0.25,
        };
        out.update_matrix();
        out
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JC69 {
    matrix: na::Matrix4<f64>,
    mu: f64,
}

impl RateMatrix for JC69 {
    fn get_matrix(&self) -> na::Matrix4<f64> {
        self.matrix
    }

    fn get_params(&self) -> Vec<f64> {
        vec![self.mu]
    }

    fn update_params(&mut self, params: Vec<f64>) {
        self.mu = params[0];
    }

    fn update_matrix(&mut self) {
        self.matrix = na::Matrix4::new(
            - (3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            - (3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            - (3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            - (3.0 * self.mu) / 4.0,
        );
    }

    fn matrix_move(&self) -> Self {
        let rng = rand::thread_rng();
        let dist = Uniform::new(0.0, 1.0);
        let params = vec![dist.sample(&mut rand::thread_rng())];
        let mut new: Self = Self::default();
        new.update_params(params);
        new.update_matrix();
        new
    }
}

impl Default for JC69 {
    fn default() -> Self {
        let mut out = JC69{
            matrix: na::Matrix4::identity(),
            mu: 4.0 / 3.0,
        };
        out.update_matrix();
        out
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MGE {
    matrix: na::Matrix2<f64>,
    gain_rate: f64,
}

impl MGE {
    fn get_matrix(&self) -> na::Matrix2<f64> {
        self.matrix
    }

    fn get_params(&self) -> Vec<f64> {
        vec![self.gain_rate]
    }

    fn update_matrix(&mut self) {
        self.matrix = na::Matrix2::new(-self.gain_rate, 
            self.gain_rate,
            self.gain_rate, 
            -self.gain_rate);
    }

    fn update_params(&mut self, params: Vec<f64>) {
        self.gain_rate = params[0];
    }
}

// pub fn update_matrix<T: RateMatrix>(topology: &mut Topology,
//         accept_fn: fn(&f64, &f64) -> bool, 
//         gen_data: &mut ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>, 
//         rate_matrix: &mut T) -> () {

//         // Get current likelihood, calculating if needed
//         if topology.likelihood.is_none() {
//             topology.likelihood = Some(likelihood(&topology, gen_data));
//         }
//         let old_ll = topology.likelihood.unwrap();
//         println!("old ll: {:?}", old_ll);
//         // Generate new matrix
//         let new_mat = rate_matrix.matrix_move();

//         // Iterator over internal nodes
//         let nodes = topology.postorder_notips(topology.get_root());
//         // HashMap for potentially temporary likelihood calculations
//         let mut temp_likelihoods: HashMap<usize, ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>> = HashMap::new();

//         // Update likelihood at internal nodes
//         for node in nodes {
//             // check if in HM
//             let lchild = node.get_lchild().unwrap();
//             let rchild = node.get_rchild().unwrap();
//             let seql: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;
//             let seqr: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;

//             match (temp_likelihoods.contains_key(&lchild), temp_likelihoods.contains_key(&rchild)) {
//                 (true, true) => {
//                     seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
//                     seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
//                 },
//                 (true, false) => {
//                     seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
//                     seqr = slice_data(rchild, &gen_data);
//                 },
//                 (false, true) => {
//                     seql = slice_data(lchild, &gen_data);
//                     seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
//                 },
//                 (false, false) => {
//                     seql = slice_data(lchild, &gen_data);
//                     seqr = slice_data(rchild, &gen_data);
//                 },
//             };

//             let node_ll = node_likelihood(seql, seqr, 
//                 &matrix_exp(&new_mat.get_matrix(), topology.nodes[lchild].get_branchlen()),
//                 &matrix_exp(&new_mat.get_matrix(), topology.nodes[rchild].get_branchlen()));

//             temp_likelihoods.insert(node.get_id(), node_ll);
//         }

//         // Calculate whole new topology likelihood at root
//         let new_ll = temp_likelihoods
//         .get(&topology.get_root().get_id())
//         .unwrap()
//         .rows()
//         .into_iter()
//         .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT));

//         println!("{:?}", new_mat.get_matrix());
//         println!("new ll: {:?}", new_ll);

//         // Likelihood decision rule
//         if accept_fn(&old_ll, &new_ll) {
//             // Drain hashmap into gen_data
//             for (i, ll_data) in temp_likelihoods.drain() {
//                 gen_data.slice_mut(s![i, .., ..]).assign(&ll_data);
//             }
//             // Update likelihood
//             topology.likelihood = Some(new_ll);
//             rate_matrix.update_params(new_mat.get_params());
//             rate_matrix.update_matrix();
//         }

// }
