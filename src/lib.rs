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

    let nstr: String = String::from("(((((((((((((20:0.0005559237124905506,(18:0.0006861528539295744,(7:0.0003272353040145137,((19:0.00025082829155862905,25:0.0004406150965139729):0.00038079747821232975,(4:0.0011438935596238994,9:0.0006423351928969907):0.0002818357686905768):0.00011932188411570738):0.0000773158870672558):0.00010670953441236318):0.00009667944131959232,3:0.000765824159948059):0.00010205320788541966,11:0.001996574669283589):0.00015507020971381987,(15:0.0007456874819936607,16:0.0005219587294727793):0.0001577483845742265):0.00006260417952833508,22:0.0013071476159608165):0.00014168737395563787,1:0.0009298473062518015):0.00036278269951022337,(0:0.0017533936651583696,14:0.0019343044045621707):0.0005499810933448594):0.00028866410976663946,26:0.002536919918888372):0.00020634571061321078,(8:0.002071089023336213,17:0.0038061797752808968):0.00011298977239988664):0.0001818640161336798,((5:0.000657447421492366,27:0.000610198789974074):0.0021134939498703524,(13:0.0029180692656446667,(12:0.0016444697624472885,24:0.0019279877425944816):0.0006543882393971138):0.00020572241428982877):0.00005593218813022198):0.0008632913785652549,10:0.0019187896996885764):0.00044364187622614485,(2:0.0012526409296072239,23:0.0011674109286468862):0.0001283930204301479):0.0013985444615635147,6:0.0008410345056844609,21:0.0008875739644970391);");
    let mut new_nstr = nstr.clone();

    let full_split: Vec<&str> = nstr.split(['(', ',',')',';']).filter(|c| !c.is_empty()).collect();
    println!("{:?}", nstr);
    let mut name_dict: HashMap<usize, String> = HashMap::new();
    let mut branch_len: HashMap<usize, f64> = HashMap::new();
    let mut id: String;
    let mut bl: f64;
    let mut leaf_count = 0;
    for (i, x) in full_split.iter().enumerate() {
        let bits: Vec<&str> = x.split(':').filter(|c| !c.is_empty()).collect();
        
        bl = bits.last().unwrap().parse().unwrap();
        branch_len.insert(i, bl);

        if bits.len().eq(&2) {
            name_dict.insert(i, bits.first().unwrap().to_string());
        }

        new_nstr = new_nstr.replace(x, &format!("{}:{}", i.to_string(), bl));
        
    };

    // println!("{:?}", new_nstr);

    let rooted_nstr: String = String::from("(((((((((((((0:0.0005559237124905506,(1:0.0006861528539295744,(2:0.0003272353040145137,((3:0.00025082829155862905,4:0.0004406150965139729)5:0.00038079747821232975,(6:0.0011438935596238993,7:0.0006423351928969907)8:0.0002818357686905768)9:0.00011932188411570738)10:0.0000773158870672558)11:0.00010670953441236318)12:0.00009667944131959232,13:0.000765824159948059)14:0.00010205320788541966,15:0.001996574669283589)16:0.00015507020971381987,(17:0.0007456874819936607,18:0.0005219587294727793)19:0.0001577483845742265)20:0.00006260417952833508,21:0.0013071476159608165)22:0.00014168737395563787,23:0.0009298473062518015)24:0.00036278269951022337,(25:0.0017533936651583696,26:0.0019343044045621706)27:0.0005499810933448594)28:0.00028866410976663946,29:0.002536919918888372)30:0.00020634571061321078,(31:0.002071089023336213,32:0.0038061797752808966)33:0.00011298977239988664)34:0.0001818640161336798,((35:0.000657447421492366,36:0.000610198789974074)37:0.0021134939498703522,(38:0.0029180692656446666,(39:0.0016444697624472884,40:0.0019279877425944815)41:0.0006543882393971138)42:0.00020572241428982877)43:0.00005593218813022198)44:0.0008632913785652549,45:0.0019187896996885763)46:0.00044364187622614485,(47:0.0012526409296072238,48:0.0011674109286468862)49:0.0001283930204301479)50:0.0013985444615635147,(51:0.0008410345056844609,52:0.0008875739644970391)53:0.0001);");
    let y = newick_to_vec(&rooted_nstr, name_dict.len());
    // println!("{:?}", y);

    // println!("{:?}", name_dict);
    // println!("{:?}", branch_len);
    // println!("leaf count: {}", name_dict.len());

    // let left_start: &char = &'(';
    // let left_end: &char = &',';
    // let right_start: &char = &',';
    // let right_end: &char = &')';
    // let newick_end: &char = &';';
    // let mut leaf_idx : usize = 0;


    // for i in 0..nstr.len() {
    //     // println!("i: {i}");
    //     let ch: char = nstr.chars().nth(i).unwrap();
    //     // println!("ch: {ch}");
    //     if ch.eq(left_start) || ch.eq(right_start) {
    //         let mut j = i + 1;
    //         // println!("j: {j}");
    //         let mut jch: char = nstr.chars().nth(j).unwrap();
    //         // println!("jch: {jch}");
    //         while j < nstr.len() && jch.ne(left_start) && jch.ne(left_end) && jch.ne(right_start) &&
    //         jch.ne(right_end) && jch.ne(newick_end) {
    //             j += 1;
    //             jch = nstr.chars().nth(j).unwrap();
    //             // println!("j: {j}");
    //             // println!("jch: {jch}");
    //         }

    //         if j != (i + 1) {
    //             let mut leaf: &str = &nstr[(i + 1)..(j)];
    //             let spl: Vec<&str> = leaf.split(':').collect();
    //             name_dict.insert(leaf_idx, spl[0].to_string());
    //             let bl: f64 = spl[1].parse().unwrap();
    //             branch_len.insert(leaf_idx, bl);
    //             // println!("{:?}", leaf);
    //             // println!("{:?}", &leaf_idx.to_string());
    //             // println!("{:?}", new_nstr);
    //             new_nstr = new_nstr.replace(leaf, &leaf_idx.to_string());
    //             // println!("{:?}", new_nstr);
    //             leaf_idx += 1;
    //         }

    //     }
    // }
    // println!("{:?}", new_nstr);
    // // // let x = parse_rapidNJ_newick(&nstr);
    // // // println!("{:?}", x);
    // // // println!("{n}");
    // let y = newick_to_vec(&new_nstr, n);
    // println!("{:?}", y);
    // println!("{:?}", name_dict.len());
    // println!("{:?}", branch_len.len());

    // // let chck: bool = y.iter().enumerate().all(|(i, x)| *x <= (2 * i - 1));
    // // // println!("{:?}", check);

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

    // println!("{}", tr.get_tree_likelihood());
    // println!("{:?}", tr.newick());
    // println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        // tr.hillclimb(&q, 100);
    }
    
    // let end = Instant::now();

    // eprintln!("Done in {}s", end.duration_since(start).as_secs());
    // eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    // eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

}
