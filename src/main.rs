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
    let start = Instant::now();

    let filename = "listeria0.aln";
    // Read in sequences into GeneticData format
    let ll = create_genetic_data(filename);
    // Build tree from vector

    let mut tr = phylo2vec_quad(vec![0; 26]);
    println!("{:?}", tr.preorder(tr.get_root()).map(|el| el.index).max());
    // Define rate matrix
    let mut q: na::Matrix4<f64> = na::Matrix4::new(-2.0, 1.0, 1.0, 1.0, 
        1.0, -2.0, 1.0, 1.0,
        1.0, 1.0, -2.0, 1.0,
        1.0, 1.0, 1.0 , -2.0);

    let mut internal_nodes = GeneticData{likelihood_lists: vec![vec![Mutation(0, 0.0,0.0,0.0,0.0,)]; 53]};

    for node in tr.postorder_notips(tr.get_root()) {

        // let mut node = tr.get_node(28).unwrap();

        // println!("index: {:?}", node.index);
        // println!("children: {:?}", node.children);


        let branchlengths = (tr.get_branchlength(node.children.0.unwrap()),
                                         tr.get_branchlength(node.children.1.unwrap()));

        // println!("branchlengths: {:?}", branchlengths);

        let seq1 = match node.children.0 {
            Some(i) if i <= 26 => {ll.likelihood_lists.get(i)},
            Some(i) => {internal_nodes.likelihood_lists.get(i)},
            None => {panic!("uh oh!")}
        };

        let seq2 = match node.children.1 {
            Some(i) if i <= 26 => {ll.likelihood_lists.get(i)},
            Some(i) => {internal_nodes.likelihood_lists.get(i)},
            None => {panic!("uh oh!")}
        };

        // println!("ll1 length {}", seq1.iter().len());
        // println!("ll2 length {}", seq1.iter().len());

        // println!("ll1 {:?}", seq2);
        // println!("ll2 length {}", seq1.iter().len());

        let cb = combine_lists(seq1, seq2, branchlengths, &q);

        // println!("{:?}", cb);

        // Need to figure out how to assign this
        internal_nodes.likelihood_lists[node.index] = cb;

    }

    

    // Need to debug
    // internal_nodes.likelihood_lists[28] = combine_lists(ll.likelihood_lists.get(0),
    //                                                           ll.likelihood_lists.get() seq2, branchlengths, rate_matrix)
    // // combine_lists(ll.likelihood_lists.get(26), internal_nodes.likelihood_lists.get(28));

    // println!("{:?}", internal_nodes);
    

    // let combo = combine_lists(ll.likelihood_lists.get(0), 
    //                                          ll.likelihood_lists.get(1),
    //                                         (tr.get_branchlength(0), tr.get_branchlength(1)),
    //                                         &q);
    // println!("seq1: {:?}",ll.likelihood_lists);
    // println!("combined seq: {:?}", combo);

    // let a = Mutation(1, 0.55, 0.15, 0.15, 0.1);
    // let b = Mutation(1, 0.35, 0.25, 0.25, 0.1);

    // println!("{:?}", q);
    
    // let temp = a.likelihood(0.125, &q)
    //                       .prod(b.likelihood(0.5, &q));
    // println!("{:?}", temp);

    // println!("{:?}", ll.likelihood_lists.get(0).unwrap().get(0));
    
    // let tr2 = phylo2vec_lin(vec![0, 0, 2, 3], false);

    // println!("{:?}", tr);
    // let tr2 = phylo2vec_lin(vec![0; 500000], true);

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
