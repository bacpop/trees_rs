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
    let mut k = 0;
    for i in &tree.nodes {
        println!{"index: {}, {}", k, i};
        k += 1;
    }

    // Iterate from element 5 in node list to root
    // and print sample names
    // let ti = tree.iter(5);
    // for i in ti {
    //     println!("iter index: {i}, Node index: {}", tree.get_node(i).unwrap().sample_name);
    // }

    // Left child traversal from root
    let tl = tree.leftiter(0);
    for j in tl {
        println!("iter index: {j}, Node index: {}", tree.get_node(j).unwrap().sample_name);
    }
}
