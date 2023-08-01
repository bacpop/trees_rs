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
use crate::node::Node;
use std::collections::HashMap;
use std::thread::current;
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
    x2[8] = 2;
    x2[7] = 3;
    // let x: Vec<usize> = (0..1000).collect();

    tr = tr.update(x2);

    // println!("{}", tr.nodes.len());
    // println!("{:?}", tr.changes);
    println!("{:?}", tr.changehm);

    // let mut i: Vec<usize> = tr.changehm.keys().cloned().collect();
    let max_depth = *tr.changehm.keys().max().unwrap();
    let md: Vec<usize> = (0..=max_depth).rev().collect();
    // i.sort();
    // i.sort_by(|a, b| b.cmp(a));
    println!("max depth: {:?}", md);
    
    for current_depth in (0..=max_depth).rev() {
        println!("current depth: {:?}", current_depth);
        let mut nodes = tr.changehm.remove(&current_depth).unwrap();
        nodes.sort();
        nodes.dedup();
        println!("nodes in this depth: {:?}", nodes);

        for node in nodes {
            // Do likelihood update traversals
            println!("current node: {:?}", node);
            let parent = tr.get_parent(node).unwrap().index;
            let parent_depth = if current_depth == 0 {0} else {current_depth - 1};
            println!("parent: {:?}, at depth {}", parent, parent_depth);
            
            // Put parent into hashmap
            match tr.changehm.get(&parent_depth) {
                None => {
                    tr.changehm.insert(parent_depth, vec![parent]);},
                Some(_) => {
                    tr.changehm.get_mut(&parent_depth).unwrap().push(parent);
                },
            }
        }

    }
    
    println!("{:?}", tr.changehm);
    
    
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
