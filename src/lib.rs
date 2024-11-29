mod newick_to_vec;
mod tests;
mod iterators;
mod rate_matrix;
mod topology;
mod genetic_data;
mod moves;
mod state_data;
mod branchlength;

use rate_matrix::RateMatrix;
use state_data::create_dummy_statedata;
use topology::Topology;

use crate::newick_to_vec::*;
extern crate nalgebra as na;
pub mod cli;
use crate::cli::*;
use std::env::args;
use std::time::Instant;
use crate::genetic_data::*;
use crate::moves::*;
use rand::Rng;
use crate::iterators::Handedness;
// use crate::rate_matrix::update_matrix;
use ndarray::s;
use std::collections::HashMap;

pub fn main() {
    let args = cli_args();
    let start = Instant::now();

    // let mut tr = vector_to_tree(&random_vector(4));
    // tr.add_genetic_data(&String::from("/Users/joel/Downloads/listeria0.aln"));
    // let n_seqs = count_sequences(&args.alignment);

    let v = random_vector(28);

    let mut t: Topology = Topology::from_vec(&v);

    let p = &rate_matrix::GTR::default();
    let mut gen_data = create_genetic_data(&args.alignment, &t, &p.get_matrix());

    println!("{:?}", likelihood(&t, &gen_data));
    println!("{:?}", t.get_newick());
    println!("{:?}", t.tree_vec);

    let mge_mat = na::Matrix2::new(0.4, 0.6, 0.6, 0.4);
    let mut st = create_dummy_statedata(1, &t, &mge_mat);

    let nodes: Vec<usize> = t.postorder(t.get_root()).map(|n| n.get_id()).collect();
    for i in nodes {
        let old_len = t.nodes[i].get_branchlen();
        t.nodes[i].set_branchlen(old_len + 1.0);
    };

    pub struct TreeState<R: RateMatrix>{
        top: Topology,
        mat: R,
        ll: Option<f64>,
        changed_nodes: Option<Vec<usize>>,
    }

    pub trait TreeMove<R: RateMatrix> {
        fn generate(&self, ts: &TreeState<R>) -> TreeState<R>;
    }

    pub struct MatrixMove {}

    impl<R: RateMatrix> TreeMove<R> for MatrixMove {
        fn generate(&self, ts: &TreeState<R>) -> TreeState<R> {
            let rm = ts.mat.matrix_move();
            let changes: Vec<usize> = ts.top.postorder_notips(ts.top.get_root()).map(|n| n.get_id()).collect();
            // This is not ideal
            let new_top = Topology{
                nodes: ts.top.nodes.clone(),
                tree_vec: ts.top.tree_vec.clone(),
                likelihood: ts.top.likelihood,
            };

            TreeState{
                top: new_top,
                mat: rm,
                ll: ts.ll,
                changed_nodes: Some(changes),
            }
        }
    }

    impl<R:RateMatrix> TreeMove<R> for ExactMove {
        fn generate(&self, ts: &TreeState<R>) -> TreeState<R> {
            let new_topology = Topology::from_vec(&self.target_vector);
            let changes: Option<Vec<usize>> = ts.top.find_changes(&new_topology);
            let mat = ts.mat;
            TreeState{
                top: new_topology,
                mat: mat,
                ll: ts.ll,
                changed_nodes: changes,
            }
        }
    }

    impl<R: RateMatrix> TreeState<R> {

        pub fn likelihood(&self, gen_data: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>) -> f64 {
            let root_likelihood = gen_data.slice(s![self.top.get_root().get_id(), .., .. ]);

            root_likelihood
            .rows()
            .into_iter()
            .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT))
        }


        pub fn apply_move<T: TreeMove<R>>(mut self, 
            move_fn: T,
            accept_fn: fn(&f64, &f64) -> bool, 
            gen_data: &mut ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>) -> TreeState<R> {

            if self.ll.is_none() {
                self.ll = Some(self.likelihood(gen_data));
            }
            let old_ll = self.ll.unwrap();

            let rate_mat = self.mat.get_matrix();
            let new_ts = move_fn.generate(&self);

            // If move did nothing, return old TreeState
            if new_ts.changed_nodes.is_none() {
                return self
            }

            // Do minimal likelihood updates (and push new values into HashMap temporarily)
            let nodes = new_ts.top.changes_iter(new_ts.changed_nodes.unwrap());
            let mut temp_likelihoods: HashMap<usize, ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>> = HashMap::new();

            for node in nodes {
                // check if in HM
                let lchild = node.get_lchild().unwrap();
                let rchild = node.get_rchild().unwrap();
                let seql: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;
                let seqr: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;
    
                match (temp_likelihoods.contains_key(&lchild), temp_likelihoods.contains_key(&rchild)) {
                    (true, true) => {
                        seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
                        seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
                    },
                    (true, false) => {
                        seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
                        seqr = slice_data(rchild, &gen_data);
                    },
                    (false, true) => {
                        seql = slice_data(lchild, &gen_data);
                        seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
                    },
                    (false, false) => {
                        seql = slice_data(lchild, &gen_data);
                        seqr = slice_data(rchild, &gen_data);
                    },
                };
    
                let node_ll = node_likelihood(seql, seqr, 
                    &matrix_exp(&rate_mat, new_ts.top.nodes[lchild].get_branchlen()),
                    &matrix_exp(&rate_mat, new_ts.top.nodes[rchild].get_branchlen()));
    
                temp_likelihoods.insert(node.get_id(), node_ll);
            }

            // Calculate whole new topology likelihood at root
        let new_ll = temp_likelihoods
        .get(&new_ts.top.get_root().get_id())
        .unwrap()
        .rows()
        .into_iter()
        .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT));

        // Likelihood decision rule
        if accept_fn(&old_ll, &new_ll) {
            // Drain hashmap into gen_data
            for (i, ll_data) in temp_likelihoods.drain() {
                gen_data.slice_mut(s![i, .., ..]).assign(&ll_data);
            }
            // Update Topology
            self.top.nodes = new_ts.top.nodes;
            self.top.tree_vec = new_ts.top.tree_vec;
            self.mat = new_ts.mat;
            self.ll = Some(new_ll);
        };

        self

    }
    }
    // let mut pp = rate_matrix::GTR::default();
    // println!("{:?}", pp.get_matrix());
    // update_matrix(&mut t, always_accept, &mut gen_data, &mut pp);
    // println!("{:?}", pp.get_matrix());
    // let mv = ChildSwap{};
    // t.apply_move(mv2, hillclimb_accept, &mut gen_data, &mut p.get_matrix());

    if !args.no_optimise {
        let start = Instant::now();
        for i in 0..0 {
            println!{"Step {}", i};
            // let new_v = random_vector(27);
            // let mv = ExactMove{target_vector: new_v};
            // let mv = ChildSwap{};
            let mv = PeturbVec{n: 1};
            t.apply_move(mv, hillclimb_accept, &mut gen_data, &mut p.get_matrix());
            
        }
        let end = Instant::now();
        println!("New likelihood: {:?}", likelihood(&t, &gen_data));
        eprintln!("Done in {}s", end.duration_since(start).as_secs());
        eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    }

}
