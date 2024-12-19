//use crate::topology::Topology;
use rand::distributions::{Distribution, Uniform};
use statrs::distribution::Dirichlet;
// use crate::TreeState;
// use crate::treestate::TreeMove;

pub trait RateMatrix: Copy {
    fn update_matrix(&mut self);

    fn update_params(&mut self, params: Vec<f64>);

    fn get_matrix(&self) -> na::Matrix4<f64>;

    fn set_matrix(&mut self, mat: na::Matrix4<f64>);

    fn get_params(&self) -> Vec<f64>;

    fn matrix_move(&self) -> Self;
}

// pub struct MatrixMove {}

// impl<R: RateMatrix> TreeMove<R> for MatrixMove {
//     fn generate(&self, ts: &TreeState<R>) -> TreeState<R> {
//         let rm = ts.mat.matrix_move();
//         let changes: Vec<usize> = ts.top.postorder_notips(ts.top.get_root()).map(|n| n.get_id()).collect();
//         // This is not ideal
//         let new_top = Topology{
//             nodes: ts.top.nodes.clone(),
//             tree_vec: ts.top.tree_vec.clone(),
//         };

//         TreeState{
//             top: new_top,
//             mat: rm,
//             ll: ts.ll,
//             changed_nodes: Some(changes),
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Gtr {
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

impl RateMatrix for Gtr {
    fn get_params(&self) -> Vec<f64> {
        vec![
            self.a, self.b, self.c, self.d, self.e, self.f, self.p0, self.p1, self.p2, self.p3,
        ]
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

    fn set_matrix(&mut self, mat: na::Matrix4<f64>) {
        self.matrix = mat;
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
            -(self.c * self.p0 + self.e * self.p1 + self.f * self.p2),
        );
    }

    fn matrix_move(&self) -> Self {
        let d1 = Dirichlet::new_with_param(1.0, 6).unwrap();
        let pars = d1.sample(&mut rand::thread_rng());

        let d2 = Dirichlet::new_with_param(1.0, 4).unwrap();
        let pars2 = d2.sample(&mut rand::thread_rng());

        // let params: Vec<f64> = pars.iter().chain(pars2.iter()).map(|x| *x).collect();
        let params: Vec<f64> = pars.iter().chain(pars2.iter()).copied().collect();
        let mut new: Self = Self::default();
        new.update_params(params);
        new.update_matrix();
        new
    }
}

impl Default for Gtr {
    fn default() -> Self {
        let mut out: Gtr = Gtr {
            matrix: na::Matrix4::identity(),
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
pub struct Jc69 {
    matrix: na::Matrix4<f64>,
    mu: f64,
}

impl RateMatrix for Jc69 {
    fn get_matrix(&self) -> na::Matrix4<f64> {
        self.matrix
    }

    fn set_matrix(&mut self, mat: na::Matrix4<f64>) {
        self.matrix = mat;
    }

    fn get_params(&self) -> Vec<f64> {
        vec![self.mu]
    }

    fn update_params(&mut self, params: Vec<f64>) {
        self.mu = params[0];
    }

    fn update_matrix(&mut self) {
        self.matrix = na::Matrix4::new(
            -(3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            -(3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            -(3.0 * self.mu) / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            self.mu / 4.0,
            -(3.0 * self.mu) / 4.0,
        );
    }

    fn matrix_move(&self) -> Self {
        let dist = Uniform::new(0.0, 1.0);
        let params = vec![dist.sample(&mut rand::thread_rng())];
        let mut new = Self::default();
        new.update_params(params);
        new.update_matrix();
        new
    }
}

impl Default for Jc69 {
    fn default() -> Self {
        let mut out = Jc69 {
            matrix: na::Matrix4::identity(),
            mu: 4.0 / 3.0,
        };
        out.update_matrix();
        out
    }
}
