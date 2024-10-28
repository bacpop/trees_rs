mod newick_to_vec;
mod tests;
mod iterators;
mod rate_matrix;
mod topology;
mod genetic_data;

use rate_matrix::RateMatrix;
use topology::Topology;

use crate::newick_to_vec::*;
extern crate nalgebra as na;
pub mod cli;
use crate::cli::*;
use std::env::args;
use std::time::Instant;
use crate::genetic_data::*;

pub fn main() {
    let args = cli_args();
    let start = Instant::now();

    // let mut tr = vector_to_tree(&random_vector(4));
    // tr.add_genetic_data(&String::from("/Users/joel/Downloads/listeria0.aln"));
    let v = random_vector(27);

    // let mut tr = vector_to_tree(&v, &GTR::default());
    // tr.add_genetic_data(&args.alignment);

    let t: Topology = Topology::from_vec(&v);

    let p = &rate_matrix::GTR::default();
    let mut gen_data = create_genetic_data(&args.alignment, &t, &p.get_matrix());


    // println!("topology likelihood: {}", likelihood(&t, &gen_data));

    // tr.initialise_likelihood();
    // println!("tree likelihood {}", tr.get_tree_likelihood());
    println!("{:?}", likelihood(&t, &gen_data));
    println!("{:?}", t.get_newick());
    println!("{:?}", t.tree_vec);

    if !args.no_optimise {
        // let start = Instant::now();
        // tr.hillclimb(0);
        // let end = Instant::now();
        println!("Optimisation currently broken");
        // eprintln!("Done in {}s", end.duration_since(start).as_secs());
        // eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    }

}
