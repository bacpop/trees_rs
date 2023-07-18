mod node;
mod tree;
mod import;
mod gen_list;
mod tests;
mod phylo2vec;


use crate::gen_list::*;
use crate::phylo2vec::*;
use crate::tree::Tree;
use std::time::{Instant};
use needletail::*;


fn main() {
    let start = Instant::now();

    // let filename = "listeria0.aln";
    // let mut reader = parse_fastx_file(filename).expect("error");

    // let record = reader.next().unwrap().unwrap();
    // let seq_vec:Vec<char> = record.seq().iter().map(|l| *l as char).collect();

    // let record2 = reader.next().unwrap().unwrap();
    // let seq2: Vec<char> = record2.seq().iter().map(|l| *l as char).collect();

    // let record3 = reader.next().unwrap().unwrap();
    // let seq3: Vec<char> = record3.seq().iter().map(|l| *l as char).collect();

    // let mut out: Vec<Mutation> = create_list(&seq_vec, &seq2);
    // let mut out2: Vec<Mutation> = create_list(&seq_vec, &seq3);

    // let combined_out = combine_lists(&mut out, &mut out2);
    // println!("combined seq: {:?}", combined_out[0..25].to_vec());

    // let tr = phylo2vec_quad(vec![0, 1, 0]);
    // let tr2 = phylo2vec_lin(vec![0, 0, 2, 3], false);

    // println!("{:?}", tr);
    // let tr2 = phylo2vec_lin(vec![0; 500000], true);

    let end = Instant::now();
    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

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
