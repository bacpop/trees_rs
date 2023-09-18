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
use std::collections::HashSet;
use std::time::Instant;
extern crate nalgebra as na;

fn main() {
    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -2.0, 1.0, 1.0, 1.0, 1.0, -2.0, 1.0, 1.0, 1.0, 1.0, -2.0, 1.0, 1.0, 1.0, 1.0, -2.0,
    );
    

    let mut tr = phylo2vec_lin(vec![0; 20], false);
    tr.mutation_lists = create_dummy_genetic_data(20, 1, 100);
    let start = Instant::now();
    tr.update_likelihood_postorder(&q);

    println!("{:?}", tr.get_tree_likelihood().ln());
    
    
    let theta0: Vec<f64> = random_tree(20).iter().map(|x| *x as f64).collect();
    let oldtheta = theta0.clone();
    // println!("theta0: {:?}", phi(theta0));

    let mut k = 0;

    // Peturbation vector
    let delta = peturbation_vec(20);
    println!("delta: {:?}", delta);

    // Pi vector
    let mut pivec: Vec<f64> = phi(theta0).iter().map(|el| el.floor() + 0.5).collect();
    pivec[0] = 0.0;
    println!("pivec: {:?}", pivec);

    // // theta+/-
    let thetaplus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x + (y / 2.0)).round() as usize).collect();
    let thetaminus: Vec<usize> = pivec.iter().zip(delta.iter()).map(|(x, y)| (x - (y / 2.0)).round() as usize).collect();

    
    println!("thetaplus: {:?}", thetaplus);
    println!("thetaminus: {:?}", thetaminus);
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
    
    let ldiff = x1 - x2;    
    let ghat: Vec<f64> = delta.iter().map(|el| if !el.eq(&0.0) {el * ldiff} else {0.0}).collect();


    let a = 2.0;
    let A = 2.0;
    let alpha = 0.75;
    let ak = a / (1.0 + A + k as f64).powf(alpha);

    let newtheta: Vec<f64> = oldtheta.iter().zip(ghat.iter()).map(|(theta, g)| *theta as f64 - ak * g).collect();

    println!("ghat: {:?}", ghat);
    println!("new theta: {:?}", newtheta);


    let delta2 = peturbation_vec(20);

    let mut pivec2: Vec<f64> = phi(newtheta).iter().map(|el| el.floor() + 0.5).collect();
    pivec2[0] = 0.0;
    println!("pivec: {:?}", pivec2);

    let thetaplus2: Vec<usize> = pivec2.iter().zip(delta2.iter()).map(|(x, y)| (x + (y / 2.0)).round() as usize).collect();
    let thetaminus2: Vec<usize> = pivec2.iter().zip(delta2.iter()).map(|(x, y)| (x - (y / 2.0)).round() as usize).collect();

    
    println!("thetaplus: {:?}", thetaplus2);
    println!("thetaminus: {:?}", thetaminus2);

    // println!("{:?}", );
    let end = Instant::now();

    // To update path lengths for BME



    // let filename = "listeria0.aln";
    // tr.add_genetic_data(filename);

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

    
    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

    // println!("{:?}", tr);
    // println!("{:?}", tr2);
}
