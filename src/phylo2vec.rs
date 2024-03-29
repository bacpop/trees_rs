use std::os::unix::thread;

use crate::node::Node;
use crate::Tree;
use ndarray::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub fn phylo2vec_quad(v: Vec<usize>) -> Tree {
    let mut tree = Tree::new(v);
    let k = tree.tree_vec.len();
    let mut not_processed = [true].repeat(k);
    let mut M = Array2::<usize>::zeros((k, 3));
    let mut labels = Array2::<usize>::zeros((k + 1, k + 1));

    for i in 0..=k {
        for j in 0..=k {
            if i >= j {
                labels[[i, j]] = j;
            }
        }
    }

    // We will keep track of row maxes in this vector rather than calculating each time
    let mut rowmax: Vec<usize> = (0..=k).collect();

    for i in 0..k {
        let n = rowmax[0..k]
            .iter()
            .enumerate()
            .rposition(|(index, el)| (tree.tree_vec[index] <= *el) & not_processed[index])
            .unwrap();

        let m = labels
            .slice(s![n, ..])
            .iter()
            .position(|x| *x == tree.tree_vec[n])
            .unwrap();

        M[[i, 0]] = labels[[k, m]];
        M[[i, 1]] = labels[[k, n + 1]];

        for j in n..=k {
            rowmax[j] += 1;
            labels[[j, m]] = rowmax[j];
        }

        M[[i, 2]] = labels[[k, m]];

        not_processed[n] = false;
    }

    // Add root
    tree.add(M[[k - 1, 2]], None);

    for i in (0..k).rev() {
        tree.add(M[[i, 0]], Some(M[[i, 2]]));
        tree.add(M[[i, 1]], Some(M[[i, 2]]));
    }

    tree.max_depth = tree.max_treedepth();

    tree
}

pub fn phylo2vec_lin(v: Vec<usize>, permute: bool) -> Tree {
    let mut tree = Tree::new(v);
    let k = tree.tree_vec.len();
    let mut M = Array2::<usize>::zeros((k, 3));
    let mut labels_rowk: Vec<usize> = (0..=k).collect();
    let mut rmk = k;

    for i in 0..k {
        let n = k - i - 1;
        let m = tree.tree_vec[n];

        M[[i, 0]] = labels_rowk[m];
        M[[i, 1]] = labels_rowk[n + 1];

        rmk += 1;
        labels_rowk[m] = rmk;
        M[[i, 2]] = labels_rowk[m];
    }

    if permute {
        // Permutation of leaf labels
        tree.leaf_permutations.shuffle(&mut thread_rng());
        for i in M.iter_mut().filter(|el| **el <= k + 1) {
            *i = *tree.leaf_permutations.get(*i).unwrap_or(i);
        }
    }

    // Build tree
    tree.add(M[[k - 1, 2]], None);

    for i in (0..k).rev() {
        tree.add(M[[i, 0]], Some(M[[i, 2]]));
        tree.add(M[[i, 1]], Some(M[[i, 2]]));
    }

    // Does this still need to happen?
    tree.max_depth = tree.max_treedepth();

    tree
}

pub fn random_tree(k: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();

    vec![0; k]
        .iter()
        .enumerate()
        .map(|(i, _el)| if i > 0 { rng.gen_range(0..((2 * i) - 1)) } else { 0 })
        .collect()
}

impl Tree {
    // Updates a Tree to the tree from new_vec and records changes in self.changes HashMap
    // pub fn update_tree(&mut self, new_vec: Option<Vec<usize>>, permute: bool) {

    //     if let Some(vec) = new_vec {
    //         self.tree_vec = vec;
    //     }

    //     if permute {
    //         self.leaf_permutations.shuffle(&mut thread_rng());
    //     }

    //     let k = self.tree_vec.len();
    //     let old_nodes = self.nodes.clone();
    //     self.nodes = vec![Node::default(); 2 * k + 1];

    //     let mut M = Array2::<usize>::zeros((k, 3));
    //     let mut labels_rowk: Vec<usize> = (0..=k).collect();
    //     let mut rmk = k;

    //     // Build M for new vector
    //     for i in 0..k {
    //         let n = k - i - 1;
    //         let m = self.tree_vec[n];

    //         M[[i, 0]] = labels_rowk[m];
    //         M[[i, 1]] = labels_rowk[n + 1];

    //         rmk += 1;
    //         labels_rowk[m] = rmk;
    //         M[[i, 2]] = labels_rowk[m];
    //     }

    //     // Update with leaf permutations, these are from the old tree or may have 
    //     // been newly shuffled above
    //     for i in M.iter_mut().filter(|el| **el <= k + 1) {
    //         *i = *self.leaf_permutations.get(*i).unwrap_or(i);
    //     }

    //     self.add(M[[k - 1, 2]], None);

    //     for i in (0..k).rev() {
    //         if old_nodes.get(M[[i, 0]]).unwrap().parent != Some(M[[i, 2]]) {
    //             let d = self.get_node(M[[i, 2]]).unwrap().depth;

    //             match self.changes.get(&d) {
    //                 None => {
    //                     self.changes.insert(d, vec![M[[i, 2]]]);
    //                 }
    //                 Some(_) => {
    //                     self.changes.get_mut(&d).unwrap().push(M[[i, 2]]);
    //                 }
    //             }
    //         }

    //         self.add(M[[i, 0]], Some(M[[i, 2]]));

    //         if old_nodes.get(M[[i, 1]]).unwrap().parent != Some(M[[i, 2]]) {
    //             let d = self.get_node(M[[i, 2]]).unwrap().depth;

    //             match self.changes.get(&d) {
    //                 None => {
    //                     self.changes.insert(d, vec![M[[i, 2]]]);
    //                 }
    //                 Some(_) => {
    //                     self.changes.get_mut(&d).unwrap().push(M[[i, 2]]);
    //                 }
    //             }
    //         }
    //         self.add(M[[i, 1]], Some(M[[i, 2]]));
    //     }
    // }

    pub fn update_quad(&mut self, new_vec: Vec<usize>) {

        let new_tree: Tree = phylo2vec_quad(new_vec);
        let k: usize = new_tree.nodes.len();
        let mut old_parent: Option<usize>;
        let mut new_parent: Option<usize>;

        for i in (0..k).rev() {
            old_parent = self.get_node(i).unwrap().parent;
            new_parent = new_tree.get_node(i).unwrap().parent;

            if old_parent.ne(&new_parent) {
                let d = new_tree.get_node(i).unwrap().depth;

                match self.changes.get(&d) {
                    None => {self.changes.insert(d, vec![i]);},
                    Some(_) => {self.changes.get_mut(&d).unwrap().push(i);}
                }
            }
        }

        self.tree_vec = new_tree.tree_vec;
        self.nodes = new_tree.nodes;

    }

}
