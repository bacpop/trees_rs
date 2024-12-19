mod branchlength;
mod genetic_data;
mod iterators;
mod moves;
mod newick_to_vec;
mod rate_matrix;
mod state_data;
mod tests;
mod topology;
mod treestate;

use rate_matrix::RateMatrix;
use topology::Topology;
use treestate::*;
// use treestate::{TreeState, hillclimb_accept, always_accept};

use crate::newick_to_vec::*;
extern crate nalgebra as na;
pub mod cli;
use crate::cli::*;
use crate::genetic_data::*;
use crate::moves::*;
use crate::topology::from_vec;
use std::time::Instant;

pub fn main() {
    let args = cli_args();

    // let mut tr = vector_to_tree(&random_vector(4));
    // tr.add_genetic_data(&String::from("/Users/joel/Downloads/listeria0.aln"));
    // let n_seqs = count_sequences(&args.alignment);

    let tree_vec: Vec<usize> = random_vector(28);

    let t: Topology = from_vec(&tree_vec);

    let p = rate_matrix::Gtr::default();
    let mut gen_data = create_genetic_data(&args.alignment, &t, &p.get_matrix());

    let ll = t.likelihood(&gen_data);
    // let mge_mat = na::Matrix2::new(0.4, 0.6, 0.6, 0.4);
    // let mut st = create_dummy_statedata(1, &t, &mge_mat);

    let mut ts = TreeState {
        top: t,
        mat: p,
        likelihood: ll,
    };

    println!("{:?}", ts.likelihood);
    println!("{:?}", ts.top.get_newick());
    println!("{:?}", ts.top.tree_vec);

    if !args.no_optimise {
        let start = Instant::now();
        for i in 0..50 {
            println!("Iteration {}", i);
            let mv = PeturbVec { n: 10 };
            ts = apply_move(ts, mv, hillclimb_accept, &mut gen_data);
        }
        let end = Instant::now();
        println!("New likelihood: {:?}", ts.likelihood);
        eprintln!("Done in {}s", end.duration_since(start).as_secs());
        eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    }

    // let mut rng = rand::thread_rng();
    // let distr = rand::distributions::Bernoulli::new(0.5).unwrap();

    // // Generate random peturbation vector
    // let mut delta: Vec<f64> = distr.sample_iter(rng).map(|r| if r {1.0} else {-1.0}).take(ts.top.tree_vec.len()).collect();

    // let phi: Vec<f64> = ts.top.tree_vec.iter().enumerate().map(|(i, v)|{
    //     let x = *v as f64;
    //     let ind = i as f64;
    //     if x < 0.0 {
    //         0.0
    //     } else if x > 2.0 * ind - 1.0 {
    //         ind - 0.0001
    //     } else {
    //         x
    //     }
    // }).collect();

    // let mut new_v_plus: Vec<usize> = phi.iter().zip(delta.iter()).map(|(phi, delta)| (phi + 0.5 + (delta / 2.0)) as usize).collect();
    // let mut new_v_minus: Vec<usize> = phi.iter().zip(delta.iter()).map(|(phi, delta)| (phi + 0.5 - (delta / 2.0)) as usize).collect();
    // new_v_plus[0] = 0;
    // new_v_minus[0] = 0;

    // println!("{:?}", new_v_plus);
    // println!("{:?}", new_v_minus);

    // let mut tsp = TreeState{
    //     top: Topology::from_vec(&v),
    //     mat: p,
    //     ll: None,
    //     changed_nodes: None,
    // };

    // let mut tsm = TreeState{
    //     top: Topology::from_vec(&v),
    //     mat: p,
    //     ll: None,
    //     changed_nodes: None,
    // };

    // let tspi = TreeStateIter{ts: tsp, move_fn: ExactMove{target_vector: new_v_plus}, accept_fn: always_accept, gen_data: &mut gen_data}.nth(0).unwrap();
    // let tsmi = TreeStateIter{ts: tsm, move_fn: ExactMove{target_vector: new_v_minus}, accept_fn: always_accept, gen_data: &mut gen_data}.nth(0).unwrap();
}
