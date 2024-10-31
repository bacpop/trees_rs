mod newick_to_vec;
mod tests;
mod iterators;
mod rate_matrix;
mod topology;
mod genetic_data;
mod moves;

use rate_matrix::RateMatrix;
use topology::Topology;

use crate::newick_to_vec::*;
extern crate nalgebra as na;
pub mod cli;
use crate::cli::*;
use std::env::args;
use std::time::Instant;
use crate::genetic_data::*;
use crate::moves::*;

pub fn main() {
    let args = cli_args();
    let start = Instant::now();

    // let mut tr = vector_to_tree(&random_vector(4));
    // tr.add_genetic_data(&String::from("/Users/joel/Downloads/listeria0.aln"));
    let v = random_vector(27);

    // let mut tr = vector_to_tree(&v, &GTR::default());
    // tr.add_genetic_data(&args.alignment);

    let mut t: Topology = Topology::from_vec(&v);

    let p = &rate_matrix::GTR::default();
    let mut gen_data = create_genetic_data(&args.alignment, &t, &p.get_matrix());


    // println!("{:?}", slice_gen_data(1, &gen_data));

    // tr.initialise_likelihood();
    // println!("tree likelihood {}", tr.get_tree_likelihood());
    println!("{:?}", likelihood(&t, &gen_data));
    println!("{:?}", t.get_newick());
    println!("{:?}", t.tree_vec);

    // let new_v = random_vector(27);
    // let mut t2: Topology = Topology::from_vec(&new_v);

    // let mv = ExactMove{target_vector: new_v};
    // t.apply_move(mv, hillclimb_accept, &mut gen_data, &p.get_matrix());
    // println!("updated ll: {:?}, initialised ll: {:?}", likelihood(&t, &gen_data), likelihood(&t2, &gen_data));

    

    if !args.no_optimise {
        let start = Instant::now();
        for i in 0..10 {
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
