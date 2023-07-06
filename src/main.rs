mod node;
mod tree;
mod import;
mod gen_list;
mod tests;
mod phylo2vec;

use std::ops::RangeInclusive;

use crate::phylo2vec::phylo2vec;
use crate::tree::Tree;
use crate::node::Node;
use crate::gen_list::Entry;
use crate::gen_list::Sample;
// use crate::import::str2tree;
use ndarray::*;
use std::time::{Instant};


fn main() {
    let start = Instant::now();
    let tr = phylo2vec(vec![0, 1, 2]);
    // let tr = phylo2vec(vec![0; 500000]);

    let end = Instant::now();
    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());

    // println!("{:?}", tr.get_node(2));
    // println!("{:?}", tr.swap_to_right_child(2));
    println!("{:?}", tr);


    for el in tr.postorder(tr.get_root()) {
        println!("{}", el);
    }

    // for el in tr.preorder(tr.get_root()) {
    //     println!("{:?}", el);
    // }

    

    // NEED TO FIX CODE THAT ASSIGNS TIPS WHEN BUILDING TREE FROM VECTOR
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
