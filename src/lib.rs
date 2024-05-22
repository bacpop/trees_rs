mod mutation;
mod likelihoods;
mod node;
mod build_tree;
mod tests;
mod tree;
mod hillclimb;
mod tree_iterators;
mod tree_to_newick;

use crate::hillclimb::peturb_vector;
use crate::build_tree::*;
use crate::tree::Tree;
extern crate nalgebra as na;
pub mod cli;
use crate::cli::*;
use std::time::Instant;

pub fn main() {
    let args = cli_args(); 

    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -1.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0,
         1.0 / 3.0, -1.0, 1.0 / 3.0, 1.0 / 3.0,
          1.0 / 3.0, 1.0 / 3.0, -1.0, 1.0 / 3.0,
           1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, -1.0,
    );
    
    let mut tr = vector_to_tree(&random_vector(27));
    tr.add_genetic_data(&args.alignment);

    tr.initialise_likelihood(&q);

    println!("{}", tr.get_tree_likelihood());
    println!("{:?}", tr.newick());
    println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        let start = Instant::now();
        tr.hillclimb(&q, 50);
        let end = Instant::now();

        eprintln!("Done in {}s", end.duration_since(start).as_secs());
        // eprintln!("Done in {}ms", end.duration_since(start).as_millis());
        // eprintln!("Done in {}ns", end.duration_since(start).as_nanos());
    }
    


}
