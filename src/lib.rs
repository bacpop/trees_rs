mod gen_list;
mod import;
mod likelihoods;
mod node;
mod phylo2vec;
mod tests;
mod tree;
mod dspsa;

use cxx::kind;
use cxx::let_cxx_string;
use cxx::CxxString;
use rand::Rng;
use rand::random;
use regex::{RegexSet, RegexSetBuilder};

use crate::dspsa::hill_peturb;
use crate::gen_list::*;
use crate::phylo2vec::*;
use crate::tree::Tree;
use crate::likelihoods::logse;
use crate::node::Node;
use std::collections::HashMap;
use std::thread::current;
use std::time::Instant;
extern crate nalgebra as na;


pub mod cli;
use crate::cli::*;

pub fn main() {
    let args = cli_args();
    // Newick string example
    let nstr: String = String::from("(((((('9':0.00066056,'4':0.0012556):0.00030238,('25':0.00042479,'19':0.00030083):0.00039731):0.00012351,'7':0.00039526):8.5797e-05,'18':0.00068249):0.00011977,'20':0.00056004):9.6199e-05,(((((((('17':0.0045802,'8':0.0021951):0.00019116,(((('24':0.0021162,'12':0.0017684):0.00074803,'13':0.0043482):0.00026831,('27':0.00063335,'5':0.00071804):0.0024273):4.8521e-05,((('23':0.0012435,'2':0.0012676):0.00011672,('21':0.00085695,'6':0.00096101):0.001509):0.00046989,'10':0.0020611):0.00092726):0.00022606):0.00019029,'26':0.0027488):0.00030355,('14':0.0021462,'0':0.0018751):0.00061029):0.00035469,'1':0.00096653):0.00015559,'22':0.0013558):7.0943e-05,('16':0.00054383,'15':0.00076666):0.00015841):0.00016779,'11':0.0021712):9.9157e-05,'3':0.00081132);");
    
    // Chops into nodes
    let mut v: Vec<&str> = nstr.split_inclusive(|c: char| c.eq(&')') || c.eq(&',')).filter(|s| !s.is_empty() && s.ne(&";")).collect();
    // Pop off the semi-colon
    // v.pop();
    // println!("{:?}", v);

    let n_nodes = v.len();
    let mut branch_len: Vec<f64> = Vec::with_capacity(n_nodes);
    let mut name_dict: HashMap<usize, String> = HashMap::with_capacity(n_nodes);
    let internal_index = ((n_nodes - 1) / 2) + 1;
    let mut node_index: usize = 0;

    // println!("{}", n_nodes);
    // println!("{}", internal_index);

    // Iterate over nodes
    for node in v {
        let cleaned_node: String = node.replace(&['(', '\'', ')', ','], "");
        // println!("{:?}", cleaned_node);
        let split_node: Vec<&str> = cleaned_node.split(':').collect();
        // println!("{:?}", split_node);

        let node_name: String = match split_node[0].is_empty() {
            true => node_index.to_string(),
            false => split_node[0].to_string(),
        };
        name_dict.insert(node_index, node_name);

        let blen: f64 = split_node[1].parse().unwrap();
        branch_len.push(blen);
        node_index += 1;

        // Need to reconstruct new newick string using new node labels

        // let new_el: Vec<&str> = node.split_inclusive(|c: char| c.eq(&')') || c.eq(&',') || c.eq(&'(') || c.eq(&':') || c.eq(&',')).collect();

        // println!("{:?}", node);
    }
    
    // for (key, val) in name_dict.iter() {
    //     println!("key: {key} val: {val}");
    // }
    // let v = newick_to_vec(&nstr, 27);
    

    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -1.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0,
         1.0 / 3.0, -1.0, 1.0 / 3.0, 1.0 / 3.0,
          1.0 / 3.0, 1.0 / 3.0, -1.0, 1.0 / 3.0,
           1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, -1.0,
    );

    let mut tr = phylo2vec_quad(&random_tree(27));

    // let end = Instant::now();
    tr.add_genetic_data(&args.alignment);

    tr.update_likelihood_postorder(&q);

    println!("{}", tr.get_tree_likelihood());
    println!("{:?}", tr.newick());
    println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        // tr.hillclimb(&q, 100);
    }
    
    // let end = Instant::now();

    // eprintln!("Done in {}s", end.duration_since(start).as_secs());
    // eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    // eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

}
