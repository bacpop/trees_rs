mod newick_to_vec;
mod tests;
mod iterators;
mod rate_matrix;
mod topology;
mod genetic_data;
mod moves;
mod state_data;

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

    // println!("{:?}", likelihood(&t, &gen_data));
    println!("{:?}", t.get_newick());
    // println!("{:?}", t.tree_vec);

    let mv2 = ChildSwap{};
    println!("hmm");
    let nt = mv2.generate_move(&t);
    
    // for (new, old) in nt.new_topology.nodes.iter().zip(t.nodes.iter()) {
    //     println!("old: {:?}, new: {:?}", old, new);
    // }

    // t.apply_move(mv2, hillclimb_accept, &mut gen_data, &p.get_matrix());
    // let mge_mat = na::Matrix2::new(0.4, 0.6, 0.6, 0.4);
    // let mut st = create_dummy_statedata(1, &t, &mge_mat);

    if !args.no_optimise {
        let start = Instant::now();
        for i in 0..1 {
            println!{"Step {}", i};
            // let new_v = random_vector(27);
            // let mv = ExactMove{target_vector: new_v};
            let mv = PeturbVec{n: 10};
            t.apply_move(mv, hillclimb_accept, &mut gen_data, &p.get_matrix());
            
        }
        let end = Instant::now();
        println!("New likelihood: {:?}", likelihood(&t, &gen_data));
        eprintln!("Done in {}s", end.duration_since(start).as_secs());
        eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    }

}
