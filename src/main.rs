mod node;
mod tree;

use crate::tree::Tree;

fn main() {
    // Construct tree from string
    let ts = String::from("4(2(3)(1))(6(5))");
    let tree = str2tree(ts, String::from("Tree1"));

    // Print nodes in tree
    for i in &tree.nodes {
        println!{"{}", i};
    }

    // Iterate from element 5 in node list to root
    // and print sample names
    let ti = tree.iter(5);
    for i in ti {
        println!("{:?}", tree.read_node(i).unwrap().sample_name);
    }
}

pub fn str2tree(st: String, name: String) -> Tree {
    let mut tree: Tree = Tree::new(name);
    let mut par_index: Option<usize> = None;
    let mut pin: usize = 0;

    for e in st.chars() {
        if e == ')' {
            par_index = match tree.read_node(pin) {
                None => None,
                Some(n) => n.parent,
            };
            pin = match par_index {
                None => 0,
                Some(x) => x,
            };
        } else if e != '(' {

        // } else {
            // Add node
            tree.add(String::from(e), par_index);
            pin = tree.nodes.len() - 1;
            par_index = Some(pin);
        }
    }

    tree
}