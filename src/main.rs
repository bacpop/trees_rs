mod gen_list;
mod import;
mod node;
mod phylo2vec;
mod tests;
mod tree;

use crate::gen_list::*;
use crate::phylo2vec::*;
use crate::tree::Tree;
use ndarray::ArrayBase;
use ndarray::AssignElem;
use ndarray::ViewRepr;
use ndarray::array;
use needletail::*;
use std::time::Instant;
extern crate nalgebra as na;
// extern crate blas;
// extern crate openblas_src;
// use approx::assert_ulps_eq;

fn main() {
    
    let mut tr = phylo2vec_quad(vec![0, 1, 0]);
    let mut tr2 = phylo2vec_lin(vec![0, 0, 0], false);
    tr2 = tr2.update(vec![0, 1, 0]);

    println!("{:?}", tr);
    println!("{:?}", tr2);


    // let filename = "listeria0.aln";
    // Read in sequences into GeneticData format
    // let mut ll = create_genetic_data(filename);
    // let leafn: usize = ll.likelihood_lists.len() - 1;
    // Set up vector for internal nodes GeneticData
    // ll.likelihood_lists.append(&mut vec![vec![Mutation(0, 0.0,0.0,0.0,0.0,)]; leafn]);
    
    // Build tree from vector
    // let mut tr = phylo2vec_quad(vec![0; leafn]);
    
    // Define rate matrix
    // let q: na::Matrix4<f64> = na::Matrix4::new(-2.0, 1.0, 1.0, 1.0, 
    //     1.0, -2.0, 1.0, 1.0,
    //     1.0, 1.0, -2.0, 1.0,
    //     1.0, 1.0, 1.0 , -2.0);

    let start = Instant::now();

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
