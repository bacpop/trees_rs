use crate::Tree;

pub fn str2tree(input_string: String, tree_name: String) -> Tree {
    let mut tree: Tree = Tree::new(tree_name);
    let mut parent: Option<usize> = None;
    let mut parent_index: usize = 0;

    for chr in input_string.chars() {
        if chr == ')' {
            parent = match tree.get_node(parent_index) {
                None => None,
                Some(n) => n.parent,
            };
            parent_index = parent.unwrap_or(0);
        } else if chr != '(' {
            tree.add(String::from(chr), parent);
            parent_index = tree.nodes.len() - 1;
            parent = Some(parent_index);
        }
    }

    tree
}