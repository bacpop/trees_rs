mod gen_list;
mod import;
mod node;
mod phylo2vec;
mod tests;
mod tree;
mod likelihoods;

use rand::Rng;

use crate::gen_list::*;
use crate::phylo2vec::*;
use crate::tree::Tree;
use crate::node::Node;
use std::collections::HashMap;
use std::thread::current;
use std::time::Instant;
extern crate nalgebra as na;

fn main() {
    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(-2.0, 1.0, 1.0, 1.0, 
        1.0, -2.0, 1.0, 1.0,
        1.0, 1.0, -2.0, 1.0,
        1.0, 1.0, 1.0 , -2.0);
    let start = Instant::now();

    // let filename = "listeria0.aln";
    // Read in sequences into GeneticData format
    // let mut ll = create_genetic_data(filename);
    // let leafn: usize = ll.likelihood_lists.len() - 1;
    // Set up vector for internal nodes GeneticData
    // ll.likelihood_lists.append(&mut vec![vec![Mutation(0, 0.0,0.0,0.0,0.0,)]; leafn]);
    
    // Build tree from vector
    // let mut v = random_tree(100);
    let x1: Vec<usize> = (0..10).collect();
    let mut tr = phylo2vec_lin(x1, false);

    // let mut x = vec![0, 1, 1, 2];
    // x.append(&mut vec![0; leafn - 4]);
    let mut x2: Vec<usize> = (0..10).collect();
    x2[2] = 1;
    // x2[8] = 2;
    // x2[7] = 3;
    // x2[6] = 2;
    // x2[5] = 3;
    // let x: Vec<usize> = (0..1000).collect();

    tr.update_tree(x2);
    println!("{:?}", tr.changehm);

    tr.update_likelihood(&q);
    
    println!("{:?}", tr.changehm);

    let end = Instant::now();
    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

    // println!("{:?}", tr);
    // println!("{:?}", tr2);


}
