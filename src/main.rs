mod gen_list;
mod import;
mod likelihoods;
mod node;
mod phylo2vec;
mod tests;
mod tree;

use crate::gen_list::*;
use crate::phylo2vec::*;
use crate::tree::Tree;
use std::time::Instant;
extern crate nalgebra as na;

fn main() {
    // Define rate matrix
    let q: na::Matrix4<f64> = na::Matrix4::new(
        -2.0, 1.0, 1.0, 1.0, 1.0, -2.0, 1.0, 1.0, 1.0, 1.0, -2.0, 1.0, 1.0, 1.0, 1.0, -2.0,
    );
    let start = Instant::now();

    // let mut tr = phylo2vec_lin(random_tree(21), false);
    // let filename = "listeria0.aln";
    // tr.add_genetic_data(filename);

    let mut tr = phylo2vec_lin(vec![0, 0, 0], false);
    let mut tr2 = phylo2vec_lin(vec![0, 0, 1], false);

    let genetic_data =  vec![
            vec![
                Mutation(1, 1.0, 0.0, 0.0, 0.0),
                Mutation(7, 1.0, 0.0, 0.0, 0.0),
            ],
            vec![
                Mutation(1, 0.0, 1.0, 0.0, 0.0),
                Mutation(11, 1.0, 0.0, 0.0, 0.0),
            ],
            vec![
                Mutation(2, 0.0, 0.0, 1.0, 0.0),
                Mutation(3, 1.0, 0.0, 0.0, 0.0),
            ],
            vec![
                Mutation(4, 1.0, 0.0, 0.0, 0.0),
                Mutation(5, 0.0, 0.0, 0.0, 1.0),
            ],
            vec![
                Mutation(4, 0.0, 1.0, 0.0, 0.0),
                Mutation(10, 0.0, 0.0, 0.0, 1.0),
            ],
            vec![
                Mutation(4, 0.0, 0.0, 1.0, 0.0),
                Mutation(8, 0.0, 0.0, 0.0, 1.0),
            ],
            vec![
                Mutation(4, 0.0, 1.0, 0.0, 0.0),
                Mutation(7, 1.0, 0.0, 0.0, 0.0),
            ],
        ];

    
    tr.mutation_lists = genetic_data;

    tr.update_likelihood_postorder(&q);
    
    
    // // println!("{:?}", tr.mutation_lists.get(tr.get_root().unwrap().index));
    // println!("{:?}", tr.get_likelihood());
    let x = tr.get_likelihood();
    // println!("{:?}", tr.nodes);
    tr.update_tree(Some(vec![0, 0, 1]), false);
    // println!("{:?}", tr.nodes);

    // tr.update_likelihood(&q);
    // tr.update_tree(Some(vec![0, 0, 0]), false);
    // tr.update_likelihood(&q);
    // println!("old: {}, new: {}", x, tr.get_likelihood());
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

    let end = Instant::now();
    eprintln!("Done in {}s", end.duration_since(start).as_secs());
    eprintln!("Done in {}ms", end.duration_since(start).as_millis());
    eprintln!("Done in {}ns", end.duration_since(start).as_nanos());

    // println!("{:?}", tr);
    // println!("{:?}", tr2);
}
