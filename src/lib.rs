mod build_tree;
mod hillclimb;
mod likelihoods;
mod mutation;
mod node;
mod tests;
mod tree;
mod tree_iterators;
mod tree_to_newick;

use crate::build_tree::*;
use crate::tree::Tree;
use crate::tree_iterators::*;
extern crate nalgebra as na;
pub mod cli;
use crate::cli::*;
use std::time::Instant;

pub fn main() {
    let args = cli_args();
    let start = Instant::now();

    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -1.0,
        1.0 / 3.0,
        1.0 / 3.0,
        1.0 / 3.0,
        1.0 / 3.0,
        -1.0,
        1.0 / 3.0,
        1.0 / 3.0,
        1.0 / 3.0,
        1.0 / 3.0,
        -1.0,
        1.0 / 3.0,
        1.0 / 3.0,
        1.0 / 3.0,
        1.0 / 3.0,
        -1.0,
    );

    // let mut tr = vector_to_tree(&random_vector(4));
    // tr.add_genetic_data(&String::from("/Users/joel/Downloads/listeria0.aln"));
    // println!("{:?}", tr.mutation_lists);
    let mut tr = vector_to_tree(&random_vector(27));
    tr.add_genetic_data(&args.alignment);

    // println!("{:?}", tr.nodes.get(4));
    tr.initialise_likelihood(&q);
    println!("{}", tr.get_tree_likelihood());
    // tr.update(&random_vector(27));
    // let y: ChangeOrder = tr.changeiter();
    // println!("{:?}", y);
    // println!("{:?}", tr.mutation_lists);
    println!("{:?}", tr.newick());
    // println!("{:?}", tr.mutation_lists.get(2));
    // println!("{:?}", tr.mutation_lists.get(3));
    println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        // let start = Instant::now();
        // tr.hillclimb(&q, 50);
        // let end = Instant::now();

        // eprintln!("Done in {}s", end.duration_since(start).as_secs());
        // eprintln!("Done in {}s", end.duration_since(start).as_millis());
    }
}
