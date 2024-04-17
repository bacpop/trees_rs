mod gen_list;
mod import;
mod likelihoods;
mod node;
mod phylo2vec;
mod tests;
mod tree;
mod dspsa;
mod newick;

use cxx::kind;
use cxx::let_cxx_string;
use cxx::CxxString;
use rand::Rng;
use rand::random;
use regex::{RegexSet, RegexSetBuilder};

use crate::dspsa::hill_peturb;
use crate::gen_list::*;
use crate::newick::newick_to_vec;
use crate::phylo2vec::*;
use crate::tree::Tree;
use crate::likelihoods::logse;
use crate::node::Node;
use std::collections::HashMap;
use std::thread::current;
use std::time::Instant;
extern crate nalgebra as na;


pub mod cli;
use crate::cli::*;

pub fn main() {
    let args = cli_args();  

    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -1.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0,
         1.0 / 3.0, -1.0, 1.0 / 3.0, 1.0 / 3.0,
          1.0 / 3.0, 1.0 / 3.0, -1.0, 1.0 / 3.0,
           1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, -1.0,
    );

    let mut tr = phylo2vec_quad(&random_tree(27));

    // let end = Instant::now();
    tr.add_genetic_data(&args.alignment);

    tr.update_likelihood_postorder(&q);

    println!("{}", tr.get_tree_likelihood());
    println!("{:?}", tr.newick());
    println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        // tr.hillclimb(&q, 100);
    }
    
    // let end = Instant::now();

    // eprintln!("Done in {}s", end.duration_since(start).as_secs());
    // eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    // eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

}
