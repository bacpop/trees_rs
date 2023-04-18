mod node;
mod tree;
mod import;

use crate::tree::Tree;
use crate::import::*;

fn main() {
    // Construct tree from string
    // let ts = String::from("4(2(3)(1))(6(5))");
    let ts = String::from("1(2(5(6))(4))(3)");
    let tree = str2tree(ts, String::from("Tree1"));

    // Print nodes in tree
    // let mut k = 0;
    // for i in &tree.nodes {
    //     println!{"index: {}, {}", k, i};
    //     k += 1;
    // }

    // Iterate from a node to the root and print each node along the way
    tree.iter(tree.get_node(3)).for_each(|node| println!("{}", node));

    // Can do things like count how many nodes to root
    println!("{}", tree.iter(tree.get_node(3)).fold(0,|acc, _node| acc + 1));
    // Or if nodes store their own likelihoods can sum up to root

    // Left child traversal from root
    tree.leftiter(tree.get_root()).for_each(|node| println!("{}", node));
    
}
