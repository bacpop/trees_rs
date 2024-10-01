mod build_tree;
mod hillclimb;
mod likelihoods;
mod mutation;
mod node;
mod tests;
mod tree;
mod tree_iterators;
mod tree_to_newick;
mod rate_matrix;

use crate::build_tree::*;
use crate::tree::Tree;
use crate::rate_matrix::GTR;
use crate::rate_matrix::JC69;
use crate::tree_iterators::*;
extern crate nalgebra as na;
pub mod cli;
use crate::cli::*;
use std::time::Instant;

pub fn main() {
    let args = cli_args();
    let start = Instant::now();

    // let mut tr = vector_to_tree(&random_vector(4));
    // tr.add_genetic_data(&String::from("/Users/joel/Downloads/listeria0.aln"));
    let mut tr = vector_to_tree(&random_vector(27), &GTR::default());
    tr.add_genetic_data(&args.alignment);

    tr.initialise_likelihood();
    println!("{}", tr.get_tree_likelihood());
    println!("{:?}", tr.newick());
    println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        let start = Instant::now();
        tr.hillclimb(1);
        let end = Instant::now();

        eprintln!("Done in {}s", end.duration_since(start).as_secs());
        eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    }
}
