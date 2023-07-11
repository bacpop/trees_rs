mod node;
mod tree;
mod import;
mod gen_list;
mod tests;
mod phylo2vec;

use std::ops::RangeInclusive;

use crate::gen_list::MutationType;
use crate::gen_list::Mutation;
use crate::gen_list::char_to_mutationtype;
use crate::gen_list::char_to_mutation;
use crate::gen_list::combine_lists;
use crate::phylo2vec::phylo2vec_lin;
use crate::phylo2vec::phylo2vec_quad;
use crate::tree::Tree;
use crate::node::Node;
// use crate::import::str2tree;
use ndarray::*;
use rand::seq::SliceRandom;
use std::time::{Instant};
use needletail::*;
use rand::{thread_rng, Rng};


fn main() {
    let start = Instant::now();

    let filename = "listeria0.aln";
    let mut reader = parse_fastx_file(&filename).expect("error");

    let ref_seq = reader.next();
    let ref_kmers = ref_seq.expect("hi");

    let record = reader.next().unwrap().unwrap();
    let seq_vec:Vec<char> = record.seq().iter().map(|l| *l as char).collect();

    let record2 = reader.next().unwrap().unwrap();
    let seq2: Vec<char> = record2.seq().iter().map(|l| *l as char).collect();

    let record3 = reader.next().unwrap().unwrap();
    let seq3: Vec<char> = record3.seq().iter().map(|l| *l as char).collect();

    // let mut out: Vec<(usize, MutationType, MutationType)> = Vec::new();
    let mut out: Vec<Mutation> = Vec::new();
    let mut out2: Vec<Mutation> = Vec::new();
    
    for (i, (s1, s2)) in seq_vec.iter().zip(seq2.iter()).enumerate() {
        if s1 != s2 {
            // println!("{}", i);
            // out.push((i, mutation_char_to_enum(*s1), mutation_char_to_enum(*s2)));
            // println!("{}", s2);
            out.push(char_to_mutation(i, s2));
        }
    }

    for (i, (s1, s2)) in seq_vec.iter().zip(seq3.iter()).enumerate() {
        if s1 != s2 {
            // println!("{}", i);
            // out.push((i, mutation_char_to_enum(*s1), mutation_char_to_enum(*s2)));
            // println!("{}", s2);
            out2.push(char_to_mutation(i, s2));
        }
    }

    // let mut i_other = 0;
    let combined_out = combine_lists(&mut out, &mut out2);
    // println!("out2: {:?}", out2);

    println!("combined seq: {:?}", combined_out);

    // let tr = phylo2vec_quad(vec![0, 0, 2, 3]);
    // let tr2 = phylo2vec_lin(vec![0, 0, 2, 3], false);
    // let tr2 = phylo2vec_lin(vec![0; 500000], true);

    let end = Instant::now();
    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());

    // println!("{:?}", tr);
    // println!("{:?}", tr2);


    // for el in tr.postorder(tr.get_root()) {
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
