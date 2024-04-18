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