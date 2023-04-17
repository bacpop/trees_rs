mod node;
mod tree;

use crate::tree::Tree;

fn main() {
    // Construct tree from string
    let ts = String::from("4(2(3)(1))(6(5))");
    let tree = str2tree(ts, String::from("Tree1"));
    println!("{:?}", tree.read_parent(4).unwrap().children);

    // Print nodes in tree
    for i in &tree.nodes {
        println!{"{:?}", i};
    }

    // Iterate from element 5 in node list to root
    // and print sample names
    let ti = tree.iter(5);
    for i in ti {
        println!("{:?}", tree.read_node(i).unwrap().sample_name);
    }
}

