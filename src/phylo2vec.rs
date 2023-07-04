use crate::Tree;
use ndarray::*;

pub fn phylo2vec(v: Vec<usize>) -> Tree {

    let mut tree = Tree::new(v);    
    let k = tree.tree_vec.len();
    let mut not_processed = vec![true].repeat(k);
    let mut M = Array2::<usize>::zeros((k, 3));
    let mut labels = Array2::<usize>::zeros((k + 1, k + 1));

    for i in 0..=k {
        for j in 0..=k {
            if(i >= j){
                labels[[i, j]] = j;
            }
        }
    }

    // println!("{}", labels);

    // We will keep track of row maxes in this vector rather than calculating each time
    let mut rowmax: Vec<usize> = (0..=k).collect();

    for i in 0..k {
        
        let n = rowmax[0..k].iter()
                                   .enumerate()
                                   .rposition(| (i, el) | {
                                        (tree.tree_vec[i] <= *el) & not_processed[i]})
                                   .unwrap();

        // println!("n: {}", n);

        let m = labels.slice(s![n, ..])
                             .iter()
                             .position(|x | *x == tree.tree_vec[n])
                             .unwrap();

        // println!("{:?}", labels.slice(s![n, ..]));
        // println!("tree_vec[n] = {}", tree.tree_vec[n]);
        // println!("m: {}", m);

        M[[i, 0]] = labels[[k, m]];
        M[[i, 1]] = labels[[k, n + 1]];
        
        for j in n..=k { 
            rowmax[j] += 1;
            labels[[j, m]] = rowmax[j];
        }

        M[[i, 2]] = labels[[k, m]];

        // println!("M: {}", M);
        // println!("labels: {}", labels);

        not_processed[n] = false;
    }

    // println!("{:?}", rowmax);
    // Add root
    tree.add(M[[k - 1, 2]], None);

    for i in (0..k).rev() {
        tree.add(M[[i, 0]], Some(M[[i, 2]]));
        tree.add(M[[i, 1]], Some(M[[i, 2]]));
    }

    return tree
}