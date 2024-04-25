mod gen_list;
mod import;
mod likelihoods;
mod node;
mod phylo2vec;
mod tests;
mod tree;
mod dspsa;
mod newick;

use cxx::kind;
use cxx::let_cxx_string;
use cxx::CxxString;
use rand::Rng;
use rand::random;
use regex::{RegexSet, RegexSetBuilder};

use crate::dspsa::hill_peturb;
use crate::gen_list::*;
use crate::newick::newick_to_vec;
use crate::newick::parse_rapidNJ_newick;
use crate::phylo2vec::*;
use crate::tree::Tree;
use crate::likelihoods::logse;
use crate::node::Node;
use std::collections::HashMap;
use std::iter::SkipWhile;
use std::thread::current;
use std::thread::park_timeout_ms;
use std::time::Instant;
extern crate nalgebra as na;


pub mod cli;
use crate::cli::*;

pub fn main() {
    let args = cli_args(); 

    let mut nstr: String = String::from("(((((((((((((20:0.0005559237124905506,(18:0.0006861528539295744,(7:0.0003272353040145137,((19:0.00025082829155862905,25:0.0004406150965139729):0.00038079747821232975,(4:0.0011438935596238994,9:0.0006423351928969907):0.0002818357686905768):0.00011932188411570738):0.0000773158870672558):0.00010670953441236318):0.00009667944131959232,3:0.000765824159948059):0.00010205320788541966,11:0.001996574669283589):0.00015507020971381987,(15:0.0007456874819936607,16:0.0005219587294727793):0.0001577483845742265):0.00006260417952833508,22:0.0013071476159608165):0.00014168737395563787,1:0.0009298473062518015):0.00036278269951022337,(0:0.0017533936651583696,14:0.0019343044045621707):0.0005499810933448594):0.00028866410976663946,26:0.002536919918888372):0.00020634571061321078,(8:0.002071089023336213,17:0.0038061797752808968):0.00011298977239988664):0.0001818640161336798,((5:0.000657447421492366,27:0.000610198789974074):0.0021134939498703524,(13:0.0029180692656446667,(12:0.0016444697624472885,24:0.0019279877425944816):0.0006543882393971138):0.00020572241428982877):0.00005593218813022198):0.0008632913785652549,10:0.0019187896996885764):0.00044364187622614485,(2:0.0012526409296072239,23:0.0011674109286468862):0.0001283930204301479):0.0013985444615635147,6:0.0008410345056844609,21:0.0008875739644970391);");
    let mut new_nstr = nstr.clone();

    // This section re-labels all the nodes, making sure that leaves have uninterrupted lower numbers than internal nodes
    // It also pulls out branch lengths and keeps them (using the new labelling scheme)
    // The result is a new re-labelled Newick string without branch lengths

    let full_split: Vec<&str> = nstr.split(['(', ',',')',';']).filter(|c| !c.is_empty()).collect();
    let mut name_dict: HashMap<usize, String> = HashMap::new();
    let mut branch_len: HashMap<usize, f64> = HashMap::new();
    let mut bl: f64;
    let mut leaf_idx: usize = 0;
    let mut internal_idx: usize = 28;
    let mut idx: usize;

    for (i, x) in full_split.iter().enumerate() {
        // Split this node into (possibly non-existant) label and branch length
        let bits: Vec<&str> = x.split(':').filter(|c| !c.is_empty()).collect();

        // Depending on size of split vector we know if we have a labelled (leaf) node or not
        if bits.len().eq(&2) {
            idx = leaf_idx;
            leaf_idx += 1;
            name_dict.insert(idx, bits.first().unwrap().to_string());
        } else {
            idx = internal_idx;
            internal_idx += 1;
        }

        // Save branch length
        bl = bits.last().unwrap().parse().unwrap();
        branch_len.insert(i, bl);

        // Put new label into new string and replace branch length
        new_nstr = new_nstr.replace(x, &format!("{}", idx.to_string()));
    };

    // This section tries to solve the polytomy at the root of rapidNJ trees
    // By splicing in some brackets and naming more internal nodes

    // Add first end bracket before last comma in string
    let firstcom = new_nstr.rfind(',').unwrap();
    new_nstr.insert(firstcom, ')');
    // Give an internal node label after this new bracket
    let mut nstr: String = vec![&new_nstr[0..=firstcom], &internal_idx.to_string(), &new_nstr[firstcom+1..new_nstr.len()]].join("");
    internal_idx += 1;
    // Find last closing bracket in string
    let firstbrack = nstr.rfind(')').unwrap();
    // Add root node label
    nstr = vec![&nstr[0..=firstbrack], &internal_idx.to_string(), &";"].join("");
    // Add corresponding opening bracket to start of string
    nstr.insert(0, '(');


    // This section goes through the Newick string and records the parent nodes of each node
    // so that we can build a Tree struct
    let left_start: &char = &'(';
    let left_end: &char = &',';
    let right_end: &char = &')';
    let mut current_parent: Option<usize> = None;
    let mut parent_vector: Vec<Option<usize>> = vec![None; internal_idx + 1];
    let mut idx: Option<usize> = Some(internal_idx);
    
    for i in (1..nstr.len()).rev() {

        let ch: char = nstr.chars().nth(i).unwrap();
        // println!("i: {i} and ch: {ch}");
        if ch.eq(right_end) || ch.eq(left_end) {

            if ch.eq(right_end) {
                // println!("current_parent was {:?} and is now {:?}", current_parent, idx);
                current_parent = idx;
            }

            let mut j = i - 1;
            let mut jch: char = nstr.chars().nth(j).unwrap();
            while j > 0 && jch.ne(left_start) && jch.ne(left_end) && jch.ne(right_end) {
                j -= 1;
                jch = nstr.chars().nth(j).unwrap();
            }

            if j != (i - 1) {
                let mut leaf: &str = &nstr[(j + 1)..i];
                idx = Some(leaf.parse().unwrap());
                // println!("leaf: {leaf}");
                // println!("Setting parent_vector[{:?}] to {:?}", idx, current_parent);
                parent_vector[idx.unwrap()] = current_parent;
            }
        } else if ch.eq(left_start) {
            current_parent = parent_vector[current_parent.unwrap()];
            // println!("current_parent = {:?}", current_parent);
        }
    }

    // Parent vector complete, time to build the tree by going over the vector
    let mut proto_tree: Tree = Tree {
        tree_vec: vec![0],
        nodes: vec![Node::default(); internal_idx + 1],
        max_depth: 0,
        leaf_permutations: (0..=leaf_idx).collect(),
        changes: HashMap::new(),
        mutation_lists: Vec::new(),
    };

    // Add nodes to Tree from parent vector, give correct branch length
    for (i, parent) in parent_vector.iter().enumerate().rev() {
        proto_tree.add(i, *parent);

        if let Some(lngth) = branch_len.get(&i) {
            proto_tree.nodes[i].branch_length = *lngth;
        } else {
            proto_tree.nodes[i].branch_length = 0.00001;
        }
    }

    // println!("{:?}", proto_tree.newick());
    // println!("{:?}", newick_to_vec(&proto_tree.newick(), proto_tree.count_leaves()));

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
