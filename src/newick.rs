use cxx::let_cxx_string;
use std::collections::HashMap;
use crate::tree::Tree;
use crate::node::Node;

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

pub fn tree_from_rapidNJ_newick(rjstr: String) -> Tree {

    let mut new_str: String = rjstr.clone();
    let full_split: Vec<&str> = rjstr.split(['(', ',',')',';']).filter(|c| !c.is_empty()).collect();
    let mut label_dictionary: HashMap<usize, String> = HashMap::with_capacity(full_split.len());
    let mut branch_length: HashMap<usize, f64> = HashMap::with_capacity(full_split.len());
    let mut bl: f64;
    let mut leaf_idx: usize = 0;
    let mut internal_idx: usize = ((full_split.len() + 1) / 2) + 1;
    let mut idx: usize;

    for (i, x) in full_split.iter().enumerate() {
        // Split this node into (possibly non-existant) label and branch length
        let bits: Vec<&str> = x.split(':').filter(|c| !c.is_empty()).collect();

        // Depending on size of split vector we know if we have a labelled (leaf) node or not
        if bits.len().eq(&2) {
            // If 2 parts we have a label and a length
            idx = leaf_idx;
            leaf_idx += 1;
            label_dictionary.insert(idx, bits.first().unwrap().to_string());
        } else {
            // If 1 part we assign an internal label
            idx = internal_idx;
            internal_idx += 1;
        }

        // Save branch length
        bl = bits.last().unwrap().parse().unwrap();
        branch_length.insert(i, bl);

        // Put new label into new string and replace branch length
        new_str = new_str.replace(x, &format!("{}", idx.to_string()));
    };

    // This section tries to solve the polytomy at the root of rapidNJ trees
    // By splicing in some brackets and naming more internal nodes
    // Add first end bracket before last comma in string
    let firstcom = new_str.rfind(',').unwrap();
    new_str.insert(firstcom, ')');
    // Give an internal node label after this new bracket
    let mut nstr: String = vec![&new_str[0..=firstcom], &internal_idx.to_string(), &new_str[firstcom+1..new_str.len()]].join("");
    internal_idx += 1;
    // Find last closing bracket in string
    let firstbrack = nstr.rfind(')').unwrap();
    // Add root node label
    nstr = vec![&nstr[0..=firstbrack], &internal_idx.to_string(), &";"].join("");
    // Add corresponding opening bracket to start of string
    nstr.insert(0, '(');

    // This section goes through the Newick string and records the parent nodes of each node
    // so that we can build a Tree struct
    let mut current_parent: Option<usize> = None;
    let mut parent_vector: Vec<Option<usize>> = vec![None; internal_idx + 1];
    let mut idx: Option<usize> = Some(internal_idx);

    for i in (1..nstr.len()).rev() {

        let ch: char = nstr.chars().nth(i).unwrap();

        if ch.eq(&')') || ch.eq(&',') {

            if ch.eq(&')') {
                current_parent = idx;
            }

            let mut j = i - 1;
            let mut jch: char = nstr.chars().nth(j).unwrap();
            while j > 0 && jch.ne(&'(') && jch.ne(&',') && jch.ne(&')') {
                j -= 1;
                jch = nstr.chars().nth(j).unwrap();
            }

            if j != (i - 1) {
                let mut leaf: &str = &nstr[(j + 1)..i];
                idx = Some(leaf.parse().unwrap());
                parent_vector[idx.unwrap()] = current_parent;
            }

        } else if ch.eq(&'(') {
            current_parent = parent_vector[current_parent.unwrap()];
        }
    };

    // Build the tree by going over the vector
    let mut proto_tree: Tree = Tree {
        tree_vec: vec![0],
        nodes: vec![Node::default(); internal_idx + 1],
        max_depth: 0,
        label_dictionary,
        changes: HashMap::new(),
        mutation_lists: Vec::new(),
    };

    // Add nodes to Tree from parent vector, give correct branch length
    for (i, parent) in parent_vector.iter().enumerate().rev() {
        proto_tree.add(i, *parent);

        if let Some(lngth) = branch_length.get(&i) {
            proto_tree.nodes[i].branch_length = *lngth;
        } else {
            proto_tree.nodes[i].branch_length = 0.00001;
        }
    };

    proto_tree
}

    // Newick string example
    // let nstr: String = String::from("(((((('9':0.00066056,'4':0.0012556):0.00030238,('25':0.00042479,'19':0.00030083):0.00039731):0.00012351,'7':0.00039526):8.5797e-05,'18':0.00068249):0.00011977,'20':0.00056004):9.6199e-05,(((((((('17':0.0045802,'8':0.0021951):0.00019116,(((('24':0.0021162,'12':0.0017684):0.00074803,'13':0.0043482):0.00026831,('27':0.00063335,'5':0.00071804):0.0024273):4.8521e-05,((('23':0.0012435,'2':0.0012676):0.00011672,('21':0.00085695,'6':0.00096101):0.001509):0.00046989,'10':0.0020611):0.00092726):0.00022606):0.00019029,'26':0.0027488):0.00030355,('14':0.0021462,'0':0.0018751):0.00061029):0.00035469,'1':0.00096653):0.00015559,'22':0.0013558):7.0943e-05,('16':0.00054383,'15':0.00076666):0.00015841):0.00016779,'11':0.0021712):9.9157e-05,'3':0.00081132);");
    // let mut nstr: String = String::from("(('7':0.00064945,'3':0.00085552):5.9503e-05,('9':0.00060885,'4':0.0013073):0.00019161,((((('6':0.0023043,'2':0.0011704):0.0017225,'5':0.0035144):8.4584e-05,'8':0.0024813):0.0002661,'0':0.0023835):0.00062217,'1':0.00096265):0.00052941);");
    // println!("{:?}", nstr);
    // First we need to root the rapidNJ tree
    // nstr = root_newick(nstr);

    // let mut x: String = parse_rapidNJ_newick(&nstr);
    // println!("{:?}", x);
    // let y: Vec<&str> = x.rsplitn(3, ')').collect();    
    // println!("{:?}", y);

// pub fn root_newick(mut nstr: String) -> String {
//     // To root, we need to add a ( after the correct , heading in from the left
//     // Then add another ) just before the ; at the end
//     let mut brack_count = 0;
//     let mut add_ind: usize = 0;
//     for (ind, el) in nstr.char_indices() {
//         if el.eq(&'('){
//             brack_count += 1;
//         } else if el.eq(&')') {
//             brack_count -= 1;
//         } else if el.eq(&',') && brack_count.eq(&1) {
//             add_ind = ind;
//             break;
//         }
//     }
//     nstr.insert(add_ind + 1, '(');
//     nstr.insert(nstr.len() - 1, ')');
//     println!("{:?}", nstr);
//     nstr
// }

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
    new_str.push_str(")");
    new_str.push_str(&node_index.to_string());
    new_str.push_str(":0.0001");
    new_str.push_str(";");

    new_str
}