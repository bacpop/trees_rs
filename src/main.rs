mod gen_list;
mod import;
mod node;
mod phylo2vec;
mod tests;
mod tree;

use rand::Rng;

use crate::gen_list::*;
use crate::phylo2vec::*;
use crate::tree::Tree;
use std::collections::HashMap;
use std::time::Instant;
extern crate nalgebra as na;

fn main() {
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
    x2[9] = 4;
    // x2[8] = 2;
    // x2[7] = 3;
    // let x: Vec<usize> = (0..1000).collect();

    tr = tr.update(x2);

    // println!("{}", tr.nodes.len());
    println!("{:?}", tr.changes);
    println!("{:?}", tr.changehm);

    println!("{:?}", tr.changehm.into_keys().max());
    
    // Define rate matrix
    // let q: na::Matrix4<f64> = na::Matrix4::new(-2.0, 1.0, 1.0, 1.0, 
    //     1.0, -2.0, 1.0, 1.0,
    //     1.0, 1.0, -2.0, 1.0,
    //     1.0, 1.0, 1.0 , -2.0);

    // tr.update_likelihood_postorder(tr.get_root(), &mut ll, &q);
    // tr.update_likelihood_rootward(tr.get_root(), &mut ll, &q);

    let end = Instant::now();
    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

    // println!("{:?}", tr);
    // println!("{:?}", tr2);

    // for el in tr.postorder_notips(tr.get_root()) {
    //     println!("{:?}", el.ll_list);
    // }

    // for el in tr.preorder(tr.get_root()) {
    //     println!("{:?}", el);
    // }

}
