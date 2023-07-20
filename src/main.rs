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

fn main() {
    let start = Instant::now();

    let filename = "listeria0.aln";
    let ll = create_genetic_data(filename);

    // println!("{:?}", ll.likelihood_lists.get_mut());
    
    let combo = combine_lists(ll.likelihood_lists.get(0), ll.likelihood_lists.get(1));
    // println!("seq1: {:?}",ll.likelihood_lists);
    // println!("combined seq: {:?}", combo);

    // let q = ndarray::Array2::<f64>::eye(4);
    let q = array![[-2., 1., 1., 1.], [1., -2., 1., 1.], [1., 1., -2., 1.], [1., 1., 1., -2.]];
    println!("{:?}", q);
    let mut p = ndarray::Array::<f64, _>::zeros((4, 4));


    let a = Mutation(1, 0.25, 0.25, 0.25, 0.25);
    let b: ArrayBase<ViewRepr<&f64>, ndarray::Dim<[usize; 1]>> = q.row(0);
    println!("{:?}", b);

    pub fn likelihood_sum(a: Mutation, b: ArrayBase<ViewRepr<&f64>, ndarray::Dim<[usize; 1]>>) -> Mutation {
        Mutation(a.0, a.1 * b[[0]], a.2 * b [[1]], a.3 * b[[2]], a.4 * b[[3]])
    }

    println!("{:?}", likelihood_sum(a, b));

    // println!("{:?}", ll.likelihood_lists.get(0).unwrap().get(0));
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
