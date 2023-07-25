use crate::Tree;
use ndarray::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use crate::node::Node;

pub fn phylo2vec_quad(v: Vec<usize>) -> Tree {
    let mut tree = Tree::new(v);
    let k = tree.tree_vec.len();
    let mut not_processed = vec![true].repeat(k);
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

impl Tree {
    pub fn update(mut self, new_vec: Vec<usize>) -> Tree {
        let k = self.tree_vec.len();
        let old_nodes = self.nodes;
        self.nodes = vec![Node::default(); 2 * k + 1];
        self.tree_vec = new_vec;
        let mut M = Array2::<usize>::zeros((k, 3));
        let mut labels_rowk: Vec<usize> = (0..=k).collect();
        let mut rmk = k;

        // Build M for new vector
        for i in 0..k {
            let n = k - i - 1;
            let m = self.tree_vec[n];

            M[[i, 0]] = labels_rowk[m];
            M[[i, 1]] = labels_rowk[n + 1];

            rmk += 1;
            labels_rowk[m] = rmk;
            M[[i, 2]] = labels_rowk[m];
        }

        // Update with leaf permutations from old tree
        for i in M.iter_mut().filter(|el| **el <= k + 1) {
            *i = *self.leaf_permutations.get(*i).unwrap_or(i);
        }

        // For now we're just going to fully rebuild the tree and record what nodes change
        let mut change_vec: Vec<(usize, Option<usize>, Option<usize>)> = Vec::new();

        for i in (0..k).rev() {
            // println!("Comparing {:?} to {:?}", old_nodes.get(M[[i, 0]]).unwrap().parent, Some(M[[i, 2]]));
            // println!("Comparing {:?} to {:?}", old_nodes.get(M[[i, 1]]).unwrap().parent, Some(M[[i, 2]]));

            if old_nodes.get(M[[i, 0]]).unwrap().parent != Some(M[[i, 2]]) {
                // Record
                change_vec.push((M[[i, 0]], old_nodes.get(M[[i, 0]]).unwrap().parent, Some(M[[i, 2]])));
            }
            self.add(M[[i, 0]], Some(M[[i, 2]]));
            
            if old_nodes.get(M[[i, 1]]).unwrap().parent != Some(M[[i, 2]]) {
                // Record
                change_vec.push((M[[i, 1]], old_nodes.get(M[[i, 1]]).unwrap().parent, Some(M[[i, 2]])));
            }
            self.add(M[[i, 1]], Some(M[[i, 2]]));
            
        }

        self.changes = change_vec;

        self
    }
}