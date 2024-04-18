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
use std::thread::current;
use std::time::Instant;
extern crate nalgebra as na;


pub mod cli;
use crate::cli::*;

pub fn main() {
    let args = cli_args();  

    let nstr: String = String::from("(((((((((((((20:0.0005559237124905506,(18:0.0006861528539295744,(7:0.0003272353040145137,((19:0.00025082829155862905,25:0.0004406150965139729):0.00038079747821232975,(4:0.0011438935596238994,9:0.0006423351928969907):0.0002818357686905768):0.00011932188411570738):0.0000773158870672558):0.00010670953441236318):0.00009667944131959232,3:0.000765824159948059):0.00010205320788541966,11:0.001996574669283589):0.00015507020971381987,(15:0.0007456874819936607,16:0.0005219587294727793):0.0001577483845742265):0.00006260417952833508,22:0.0013071476159608165):0.00014168737395563787,1:0.0009298473062518015):0.00036278269951022337,(0:0.0017533936651583696,14:0.0019343044045621707):0.0005499810933448594):0.00028866410976663946,26:0.002536919918888372):0.00020634571061321078,(8:0.002071089023336213,17:0.0038061797752808968):0.00011298977239988664):0.0001818640161336798,((5:0.000657447421492366,27:0.000610198789974074):0.0021134939498703524,(13:0.0029180692656446667,(12:0.0016444697624472885,24:0.0019279877425944816):0.0006543882393971138):0.00020572241428982877):0.00005593218813022198):0.0008632913785652549,10:0.0019187896996885764):0.00044364187622614485,(2:0.0012526409296072239,23:0.0011674109286468862):0.0001283930204301479):0.0013985444615635147,(6:0.0008410345056844609,21:0.0008875739644970391):0.00001);");
    let mut new_nstr = nstr.clone();
    let n = nstr.chars().filter(|c| c.eq(&')')).count();

    let left_start: &char = &'(';
    let left_end: &char = &',';
    let right_start: &char = &',';
    let right_end: &char = &')';
    let newick_end: &char = &';';
    let mut leaf_idx : usize = 0;
    let mut name_dict: HashMap<usize, String> = HashMap::with_capacity(n);

    for i in 0..nstr.len() {
        // println!("i: {i}");
        let ch: char = nstr.chars().nth(i).unwrap();
        // println!("ch: {ch}");
        if ch.eq(left_start) || ch.eq(right_start) {
            let mut j = i + 1;
            // println!("j: {j}");
            let mut jch: char = nstr.chars().nth(j).unwrap();
            // println!("jch: {jch}");
            while j < nstr.len() && jch.ne(left_start) && jch.ne(left_end) && jch.ne(right_start) &&
            jch.ne(right_end) && jch.ne(newick_end) {
                j += 1;
                jch = nstr.chars().nth(j).unwrap();
                // println!("j: {j}");
                // println!("jch: {jch}");
            }

            if j != (i + 1) {
                let mut leaf: &str = &nstr[(i + 1)..(j)];
                let spl: Vec<&str> = leaf.split(':').collect();
                name_dict.insert(leaf_idx, spl[0].to_string());
                // println!("{:?}", leaf);
                // println!("{:?}", &leaf_idx.to_string());
                // println!("{:?}", new_nstr);
                new_nstr = new_nstr.replace(leaf, &leaf_idx.to_string());
                // println!("{:?}", new_nstr);
                leaf_idx += 1;
            }

        }
    }
    // println!("{:?}", new_nstr);
    // let x = parse_rapidNJ_newick(&nstr);
    // println!("{:?}", x);
    // println!("{n}");
    let y = newick_to_vec(&new_nstr, n);
    // println!("{:?}", y);

    let check = y.iter().enumerate().all(|(i, x)| *x <= (2 * i - 1));
    // println!("{:?}", check);

    // let mut tr = phylo2vec_quad(&y);

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
