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
use ndarray::ViewRepr;
use ndarray::array;
use needletail::*;
use std::time::Instant;
extern crate nalgebra as na;
// extern crate blas;
// extern crate openblas_src;
// use approx::assert_ulps_eq;

fn main() {
    

    let filename = "listeria0.aln";
    let ll = create_genetic_data(filename);
    let tr = phylo2vec_quad(vec![0, 1, 0]);
    let mut q: na::Matrix4<f64> = na::Matrix4::new(-2.0, 1.0, 1.0, 1.0, 
        1.0, -2.0, 1.0, 1.0,
        1.0, 1.0, -2.0, 1.0,
        1.0, 1.0, 1.0 , -2.0);
    // println!("{:?}", ll.likelihood_lists.get_mut());
    
    let start = Instant::now();
    let combo = combine_lists(ll.likelihood_lists.get(0), 
                                             ll.likelihood_lists.get(1),
                                            (tr.get_branchlength(0), tr.get_branchlength(1)),
                                            &q);
    // println!("seq1: {:?}",ll.likelihood_lists);
    println!("combined seq: {:?}", combo);

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
    //     println!("{}", el);
    // }

    // for el in tr.preorder(tr.get_root()) {
    //     println!("{:?}", el);
    // }

    // NEED TO FIX CODE THAT BUILDS TREE FROM NEWICK STRING

    // // Print nodes in tree
    // let mut k = 0;
    // for i in &tree.nodes {
    //     println!{"index: {}, {}", k, i};
    //     k += 1;
    // }

    // println!("{:?}", tree.most_left_child(tree.get_root()));

    // tips.iter().map(|i| {
    //     let it: Vec<usize> = tree
    //     .iter(tree.get_node(*i))
    //     .fold(vec![0],|acc, _node| acc.len());
    // });

    //.map(|n| tree.iter(Some(n))).fold(0,|acc, _node| acc + 1);
    // println!("{:?}", tips);
    // println!("{:?}", lengths.iter().max());

    // Rudimentary way to relocate nodes by assigning new parents
    // tree.relocate(2, 5);

    // Iterate from a node to the root and print each node along the way
    // tree.iter(tree.get_node(3)).for_each(|node| println!("{}", node));

    // Can do things like count how many nodes to root
    // println!("{}", tree.iter(tree.get_node(3)).fold(0,|acc, _node| acc + 1));
    // println!("{}", tree.preorder(tree.get_root()).fold(0,|acc, _node| acc + 1));
    // Or if nodes store their own likelihoods can sum up to root

    // Preorder traversal from root
    // tree.preorder(tree.get_root()).for_each(|node| println!("{}", node));

    // Doesn't have to be from root, can preorder traverse from any node
    // tree.preorder(tree.get_node(1)).for_each(|node| println!("{}", node));
}
