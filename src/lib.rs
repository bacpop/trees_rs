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

    fn create_GTR_ratematrix(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, pv: Vec<f64>) -> na::Matrix4<f64> {
        // pv = pivec defined as (piA, piC, piG, piT)
        let mut q = na::Matrix4::new(
            -(a * pv[1] + b * pv[2] + c * pv[3]),
            a * pv[1],
            b * pv[2],
            c * pv[3],
            a * pv[0],
            -(a * pv[0] + d * pv[2] + e * pv[3]),
            d * pv[2],
            e * pv[3],
            b * pv[0],
            d * pv[1],
            -(b * pv[0] + d * pv[1] + f * pv[3]),
            f * pv[3],
            c * pv[0],
            e * pv[1],
            f * pv[2],
            -(c * pv[0] + e * pv[1] * f * pv[2]));

        let mut diag = 0.0;
        for i in 0..=3 {
            diag -= q[(i, i)] * pv[i];
        }

        q / diag
    }

    let q2 = create_GTR_ratematrix(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, vec![0.25, 0.25, 0.25, 0.25]);
    // println!("{:?}", q2);

    // let mut tr = vector_to_tree(&random_vector(4));
    // tr.add_genetic_data(&String::from("/Users/joel/Downloads/listeria0.aln"));
    let mut tr = vector_to_tree(&random_vector(27));
    tr.add_genetic_data(&args.alignment);

    tr.initialise_likelihood(&q);
    println!("{}", tr.get_tree_likelihood());
    println!("{:?}", tr.newick());
    println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        let start = Instant::now();
        tr.hillclimb(&q, 0);
        let end = Instant::now();

        eprintln!("Done in {}s", end.duration_since(start).as_secs());
        eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    }
}
