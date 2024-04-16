use cxx::let_cxx_string;
use std::collections::HashMap;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("bactrees/include/phylo2vec.hpp");
        fn doToVector(newick: Pin<&mut CxxString>, num_leaves: i32, with_mapping: bool) -> UniquePtr<CxxVector<i32>>;
    }
}

pub fn newick_to_vec(nw: &String, n_leaves: usize) -> Vec<usize>{
    let_cxx_string!(nw_cpp = nw);
    let x = ffi::doToVector(nw_cpp, n_leaves as i32, false);
    let y: Vec<usize> = x.iter().map(|el| *el as usize).collect();
    y
}

pub fn parse_rapidNJ_newick(nw: &String) -> String {
    
    // Chop the Newick string into nodes
    let v: Vec<&str> = nw.split_inclusive(|c: char| c.eq(&')') || c.eq(&','))
    .filter(|s| !s.is_empty() && s.ne(&";"))
    .collect();

    // Initiate data structures
    let n_nodes = v.len();
    let mut branch_len: Vec<f64> = Vec::with_capacity(n_nodes);
    let mut name_dict: HashMap<usize, String> = HashMap::with_capacity(n_nodes);
    let mut node_index: usize = 0;
    let mut new_newick: Vec<String> = Vec::with_capacity(n_nodes);

    // Iterate over nodes
    for node in v {
        // Cut outbrackets and commas
        let cleaned_node: String = node.replace(&['(', '\'', ')', ','], "");

        // Split into label and branch length
        let split_node: Vec<&str> = cleaned_node.split(':').collect();

        // Get old node label (an old label is assigned if no previous 
        // label due to being internal node)
        let node_name: String = match split_node[0].is_empty() {
            true => node_index.to_string(),
            false => split_node[0].to_string(),
        };
        // Put into name HashMap
        name_dict.insert(node_index, node_name);

        // Get branch length
        let blen: f64 = split_node[1].parse().unwrap();
        branch_len.push(blen);

        // Put new node labels into old newick string
        if split_node[0].is_empty() {
            let new_el: String = node.replace(":", &format!("{}{}", node_index.to_string(), ":"));
            new_newick.push(new_el);

        } else {
            let new_el: String = node.replace(&format!("'{}'", name_dict.get(&node_index).unwrap()), 
            &format!("{}", node_index.to_string()));
            new_newick.push(new_el);
        }

        // Iterate node index
        node_index += 1;
    }

    // Put newick string with new labels together
    let mut new_str: String = new_newick.join("");
    new_str.push_str(&node_index.to_string());
    new_str.push_str(":0.0001");
    new_str.push_str(";");

    new_str
}