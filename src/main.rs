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
use crate::dspsa::phi;
use crate::dspsa::piv;
use std::collections::HashSet;
use std::time::Instant;
extern crate nalgebra as na;

fn main() {
    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0,
    );

    // let p1 = na::Matrix::exp(&q);
    // println!("{:?}", p1);

    // let mut tr = phylo2vec_lin(vec![0, 0], false);

    // let genetic_data = vec![
    //     vec![
    //         Mutation(1, 1.0, 0.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(1, 0.0, 1.0, 0.0, 0.0),
    //         Mutation(2, 0.0, 1.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(1, 0.0, 1.0, 0.0, 0.0),
    //     ],
    //     vec![],
    //     vec![],
    // ];

    // tr.mutation_lists = genetic_data;

    // let mut0 = tr.mutation_lists.get(0).unwrap().get(0).unwrap().child_likelihood(&p1);
    // let mut2 = tr.mutation_lists.get(2).unwrap().get(0).unwrap().child_likelihood(&p1);
    // let mut1 = tr.mutation_lists.get(1).unwrap().get(0).unwrap().child_likelihood(&p1);
    // println!("mut0: {:?}", mut0);
    // println!("mut1: {:?}", mut1);
    // println!("mut2: {:?}",mut2);
    // let mut3 = mut0.prod(mut2);
    // println!("mut3: {:?}", mut3);
    // let mut41 = mut3.child_likelihood(&p1).prod(mut1);
    // println!("mut41: {:?}", mut41);
    // println!("mut41: {:?}", mut41.1 + mut41.2 + mut41.3 + mut41.4);




    let mut tr = phylo2vec_quad(vec![0; 1]);
    println!("{:?}", tr);

    let filename = "listeria_simple.fasta";
    tr.add_genetic_data(filename);

    

    // let start = Instant::now();
    tr.update_likelihood_postorder(&q);
    println!("{:?}", tr.mutation_lists);
    // println!("{:?}", tr.mutation_lists.get(tr.get_root().unwrap().index));

    println!("{:?}", tr.get_tree_likelihood());
    // println!("{:?}", tr.get_tree_log_likelihood());
    // // println!("{:?}", tr.get_root().unwrap().index);
    // // println!("{:?}", tr.mutation_lists.get(42));
    
    
    // let mut theta: Vec<f64> = tr.tree_vec.iter().map(|x| *x as f64).collect();
    // let n = theta.len();

    // let a = 2.0;
    // let A = 2.0;
    // let alpha = 0.75;

    // let mut llvec: Vec<f64> = Vec::new();

    // for k in 0..=1000 {
    //     // println!("k: {:?}", k);
    //     // println!("theta: {:?}", theta);

    //     // Peturbation vector
    //     let delta = peturbation_vec(n);
    //     // println!("delta: {:?}", delta);

    //     // Pi vector
    //     let pivec: Vec<f64> = piv(&theta);
    //     // println!("pivec: {:?}", pivec);

    //     // theta+/-
    //     let thetaplus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x + (y / 2.0)).round() as usize).collect();
    //     let thetaminus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x - (y / 2.0)).round() as usize).collect();

    //     // println!("thetaplus: {:?}", thetaplus);
    //     // println!("thetaminus: {:?}", thetaminus);

    //     // Calculate likelihood at theta trees
    //     tr.update_tree(Some(thetaplus), false);
    //     // println!("tree changes: {:?}", tr.changes);
    //     tr.update_likelihood(&q);
    //     let x1 = tr.get_tree_likelihood();
    //     // println!("thetaplus ll: {:?}", x1);

    //     tr.update_tree(Some(thetaminus), false);
    //     // println!("tree changes: {:?}", tr.changes);
    //     tr.update_likelihood(&q);
    //     let x2 = tr.get_tree_likelihood();
    //     // println!("thetaminus ll: {:?}", x2);
        
    //     // Calculations to work out new theta
    //     let ldiff = x1 - x2;    
    //     let ghat: Vec<f64> = delta.iter().map(|el| if !el.eq(&0.0) {el * ldiff} else {0.0}).collect();

    //     let ak = a / (1.0 + A + k as f64).powf(alpha);

    //     theta = theta.iter().zip(ghat.iter()).map(|(theta, g)| *theta as f64 - ak * g).collect();

    //     llvec.push(x1);

    //     // println!("ghat: {:?}", ghat);
    
    // }

    // let out: Vec<f64> = phi(&theta).iter().map(|x| x.round()).collect();
    // println!("final theta: {:?}", out);
    // let end = Instant::now();


    // println!("{:?}", &llvec[0..5]);
    // println!("{:?}", &llvec[995..1000]);

    // eprintln!("Done in {}s", end.duration_since(start).as_secs());
    // eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    // eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

}
