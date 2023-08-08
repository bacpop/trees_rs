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

    // let filename = "listeria0.aln";

    // Build tree from vector
    // let mut v = random_tree(100);
    let mut tr = phylo2vec_lin(vec![0, 0], false);
    // tr.add_genetic_data(filename);

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
        ];

    // println!("{:?}", tr);
    tr.mutation_lists = genetic_data;

    // for n in tr.postorder_notips(tr.get_root()) {
    //     println!("{:?}", n);
    //     // println!{{}}
    //     // println!("{:?}", genetic_data.likelihood_lists.get(n.index));
    // }

    let x = combine_lists(tr.mutation_lists.get(3), tr.mutation_lists.get(4), (1.0, 1.0), &q);

    println!("{:?}", x);

    let p = na::Matrix::exp(&q);
    
    let mutation = Mutation(1, 0.15, 0.5, 0.25, 0.1);
    let y = mutation.likelihood(&p);
    let muts2 = Mutation(1, 0.3, 0.1, 0.3, 0.1);
    let y2 = muts2.likelihood(&p);

    let outcome = y.prod(y2);
    

    let y1 = y.1 * y2.1;

    // println!("{:?}", tr.mutation_lists);
    // println!("{:?}", tr.nodes);
    tr.update_likelihood_postorder(&q);
    
    println!("{:?}", tr.mutation_lists.get(tr.get_root().unwrap().index));
    println!("{:?}", tr.get_likelihood().ln());

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
