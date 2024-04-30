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
use crate::newick::tree_from_rapidNJ_newick;
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
    
    let test_tr = tree_from_rapidNJ_newick(nstr);
    // println!("{:?}", test_tr);

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
