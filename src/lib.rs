mod gen_list;
mod import;
mod likelihoods;
mod node;
mod phylo2vec;
mod tests;
mod tree;
mod dspsa;

use rand::Rng;
use rand::random;

use crate::dspsa::peturbation_vec;
use crate::gen_list::*;
use crate::phylo2vec::*;
use crate::tree::Tree;
use crate::likelihoods::logse;
use crate::node::Node;
use crate::dspsa::phi;
use crate::dspsa::piv;
use std::collections::HashSet;
use std::thread::current;
use std::time::Instant;
extern crate nalgebra as na;

pub mod cli;
use crate::cli::*;

pub fn main() {
    let args = cli_args();

    let start = Instant::now();

    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -1.0, 1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0,
         1.0 / 3.0, -1.0, 1.0 / 3.0, 1.0 / 3.0,
          1.0 / 3.0, 1.0 / 3.0, -1.0, 1.0 / 3.0,
           1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, -1.0,
    );

    // let mut tr = phylo2vec_quad(vec![0, 0, 0]);
    let mut tr = phylo2vec_quad(random_tree(27));
    // let filename = "listeria0.aln";
    tr.add_genetic_data(&args.alignment);

    tr.update_likelihood_postorder(&q);

    // println!("{:?}", tr.mutation_lists);
    println!("{}", tr.get_tree_likelihood());
    println!("{:?}", tr.newick());
    println!("{:?}", tr.tree_vec);

    if !args.no_optimise {
        let mut theta: Vec<f64> = tr.tree_vec.iter().map(|x| *x as f64).collect();
        let n = theta.len();

        let a = 2.0;
        let A = 2.0;
        let alpha = 0.75;
        // let k = 0;

        let mut llvec: Vec<f64> = Vec::new();

        let start = Instant::now();
        for k in 0..=200 {
            println!("k: {:?}", k);
            // println!("theta: {:?}", theta);

        //     // Peturbation vector
            let delta = peturbation_vec(n);
            // println!("delta: {:?}", delta);

        //     // Pi vector
            let pivec: Vec<f64> = piv(&theta);
        //     // println!("pivec: {:?}", pivec);

        //     // theta+/-
            let thetaplus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x + (y / 2.0)).round() as usize).collect();
            let thetaminus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x - (y / 2.0)).round() as usize).collect();

        //     // println!("thetaplus: {:?}", thetaplus);
        //     // println!("thetaminus: {:?}", thetaminus);

        //     // Calculate likelihood at theta trees
            tr.update_tree(Some(thetaplus), false);
        //     // println!("tree changes: {:?}", tr.changes);
            tr.update_likelihood(&q);
            let x1 = tr.get_tree_likelihood();
        //     // println!("thetaplus ll: {:?}", x1);

            tr.update_tree(Some(thetaminus), false);
        //     // println!("tree changes: {:?}", tr.changes);
            tr.update_likelihood(&q);
            let x2 = tr.get_tree_likelihood();
        //     // println!("thetaminus ll: {:?}", x2);

        //     // Calculations to work out new theta
            let ldiff = x1 - x2; 
            let ghat: Vec<f64> = delta.iter().map(|el| if !el.eq(&0.0) {el * ldiff} else {0.0}).collect();

            let ak = a / (1.0 + A + k as f64).powf(alpha);

            theta = theta.iter().zip(ghat.iter()).map(|(theta, g)| *theta as f64 - ak * g).collect();

            llvec.push(x1);

        //     // println!("ghat: {:?}", ghat);

        }

        let out: Vec<f64> = phi(&theta).iter().map(|x| x.round()).collect();
        println!("final theta: {:?}", out);

        println!("{:?}", &llvec);
        // println!("{:?}", &llvec[95..100]);
    }
    let end = Instant::now();

    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

}