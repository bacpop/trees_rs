use crate::Tree;

pub fn str2tree(st: String, name: String) -> Tree {
    let mut tree: Tree = Tree::new(name);
    let mut par_index: Option<usize> = None;
    let mut pin: usize = 0;

    for e in st.chars() {
        if e == ')' {
            par_index = match tree.get_node(pin) {
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