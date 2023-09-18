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
        -2.0, 1.0, 1.0, 1.0, 1.0, -2.0, 1.0, 1.0, 1.0, 1.0, -2.0, 1.0, 1.0, 1.0, 1.0, -2.0,
    );

    let mut tr = phylo2vec_quad(random_tree(21));
    let filename = "listeria0.aln";
    tr.add_genetic_data(filename);

    let start = Instant::now();
    tr.update_likelihood_postorder(&q);

    println!("{:?}", tr.get_tree_likelihood().ln());
    
    let mut theta: Vec<f64> = tr.tree_vec.iter().map(|x| *x as f64).collect();
    let n = theta.len();

    for k in 0..=1000 {
        println!("k: {:?}", k);
        println!("theta: {:?}", theta);

        // Peturbation vector
        let delta = peturbation_vec(n);
        println!("delta: {:?}", delta);

        // Pi vector
        let pivec: Vec<f64> = piv(&theta);
        println!("pivec: {:?}", pivec);

        // theta+/-
        let thetaplus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x + (y / 2.0)).round() as usize).collect();
        let thetaminus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x - (y / 2.0)).round() as usize).collect();

        println!("thetaplus: {:?}", thetaplus);
        println!("thetaminus: {:?}", thetaminus);

        // Calculate likelihood at theta trees
        tr.update_tree(Some(thetaplus), false);
        // println!("tree changes: {:?}", tr.changes);
        tr.update_likelihood(&q);
        let x1 = tr.get_tree_likelihood().ln();
        println!("thetaplus ll: {:?}", x1);

        tr.update_tree(Some(thetaminus), false);
        // println!("tree changes: {:?}", tr.changes);
        tr.update_likelihood(&q);
        let x2 = tr.get_tree_likelihood().ln();
        println!("thetaminus ll: {:?}", x2);
        
        // Calculations to work out new theta
        let ldiff = x1 - x2;    
        let ghat: Vec<f64> = delta.iter().map(|el| if !el.eq(&0.0) {el * ldiff} else {0.0}).collect();

        let a = 2.0;
        let A = 2.0;
        let alpha = 0.75;
        let ak = a / (1.0 + A + k as f64).powf(alpha);

        theta = theta.iter().zip(ghat.iter()).map(|(theta, g)| *theta as f64 - ak * g).collect();

        println!("ghat: {:?}", ghat);
    
    }

    let out: Vec<f64> = phi(&theta).iter().map(|x| x.round()).collect();
    println!("final theta: {:?}", out);
    let end = Instant::now();

    // To update path lengths for BME

    // let mut tr = phylo2vec_lin(vec![0, 0, 0], false);
    // let mut tr2 = phylo2vec_lin(vec![0, 0, 1], false);
    // let mut x = tr.tree_vec.clone();
    // *x.get_mut(499999).unwrap() += 1;
    // tr.update_tree(Some(x), false);
    // tr.update_likelihood(&q);
    // println!("{:?}", tr.get_tree_likelihood().ln());
    // let genetic_data = vec![
    //     vec![
    //         Mutation(1, 1.0, 0.0, 0.0, 0.0),
    //         Mutation(7, 1.0, 0.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(1, 0.0, 1.0, 0.0, 0.0),
    //         Mutation(11, 1.0, 0.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(2, 0.0, 0.0, 1.0, 0.0),
    //         Mutation(3, 1.0, 0.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(4, 1.0, 0.0, 0.0, 0.0),
    //         Mutation(5, 0.0, 0.0, 0.0, 1.0),
    //     ],
    //     vec![
    //         Mutation(4, 0.0, 1.0, 0.0, 0.0),
    //         Mutation(10, 0.0, 0.0, 0.0, 1.0),
    //     ],
    //     vec![
    //         Mutation(4, 0.0, 0.0, 1.0, 0.0),
    //         Mutation(8, 0.0, 0.0, 0.0, 1.0),
    //     ],
    //     vec![
    //         Mutation(4, 0.0, 1.0, 0.0, 0.0),
    //         Mutation(7, 1.0, 0.0, 0.0, 0.0),
    //     ],
    // ];

    // tr.mutation_lists = genetic_data;

    // tr.update_likelihood_postorder(&q);


    // // // println!("{:?}", tr.mutation_lists.get(tr.get_root().unwrap().index));
    // println!("{:?}", tr.get_tree_likelihood());
    
    // tr.update_tree(Some(vec![0, 0, 1]), false);
    // tr.update_likelihood(&q);
    // println!("{:?}", tr.get_tree_likelihood());

    // tr.update_tree(Some(vec![0, 0, 0]), false);
    // tr.update_likelihood(&q);
    // println!("{:?}", tr.get_tree_likelihood());
    // tr.update_tree(Some(vec![0, 0, 0]), false);
    // tr.update_likelihood(&q);
    // // tr.update_likelihood(&q);

    // println!("{:?}", tr.changes.keys().max().unwrap());
    // println!("{:?}", tr.get_likelihood());

    // tr.update_likelihood_postorder(tr.get_root(), &mut ll, &q);

    // println!("{:?}", &ll.likelihood_lists.get(30).unwrap()[0..10]);

    // let mut x = vec![0, 1, 1, 2];
    // x.append(&mut vec![0; leafn - 4]);
    // let mut x2: Vec<usize> = (0..10).collect();
    // x2[2] = 1;
    // x2[8] = 2;
    // x2[7] = 3;
    // x2[6] = 2;
    // x2[5] = 3;
    // let x: Vec<usize> = (0..1000).collect();

    // tr.update_tree(x2);
    // println!("{:?}", tr.changes);

    // tr.update_likelihood(&q);

    // println!("{:?}", tr.changes);

    
    // eprintln!("Done in {}s", end.duration_since(start).as_secs());
    // eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    // eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

    // println!("{:?}", tr);
    // println!("{:?}", tr2);
}
