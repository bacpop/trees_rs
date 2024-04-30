use crate::Tree;
use crate::node::Node;
use cxx::let_cxx_string;
use ndarray::*;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::HashMap;

////////////////////////////////////////////////
// Build a Tree struct from an integer vector //
////////////////////////////////////////////////

pub fn vector_to_tree(v: &[usize]) -> Tree {
    let mut tree = Tree::new(v);
    let mut sub_vec = v.to_vec();
    sub_vec.remove(0);
    let k = sub_vec.len();
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
            .rposition(|(index, el)| (sub_vec[index] <= *el) & not_processed[index])
            .unwrap();

        let m = labels
            .slice(s![n, ..])
            .iter()
            .position(|x| *x == sub_vec[n])
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

///////////////////////////////////////////////
// Build a Tree struct from an newick string //
///////////////////////////////////////////////
pub fn newick_to_tree(rjstr: String) -> Tree {

    let mut new_str: String = rjstr.clone();
    let full_split: Vec<&str> = rjstr.split(['(', ',',')',';']).filter(|c| !c.is_empty()).collect();
    let mut label_dictionary: HashMap<usize, String> = HashMap::with_capacity(full_split.len());
    let mut branch_length: HashMap<usize, f64> = HashMap::with_capacity(full_split.len());
    let mut bl: f64;
    let mut leaf_idx: usize = 0;
    let mut internal_idx: usize = ((full_split.len() + 1) / 2) + 1;
    let mut idx: usize;

    for (i, x) in full_split.iter().enumerate() {
        // Split this node into (possibly non-existant) label and branch length
        let bits: Vec<&str> = x.split(':').filter(|c| !c.is_empty()).collect();

        // Depending on size of split vector we know if we have a labelled (leaf) node or not
        if bits.len().eq(&2) {
            // If 2 parts we have a label and a length
            idx = leaf_idx;
            leaf_idx += 1;
            label_dictionary.insert(idx, bits.first().unwrap().to_string());
        } else {
            // If 1 part we assign an internal label
            idx = internal_idx;
            internal_idx += 1;
        }

        // Save branch length
        bl = bits.last().unwrap().parse().unwrap();
        branch_length.insert(i, bl);

        // Put new label into new string and replace branch length
        new_str = new_str.replace(x, &format!("{}", idx.to_string()));
    };

    // This section tries to solve the polytomy at the root of rapidNJ trees
    // By splicing in some brackets and naming more internal nodes
    // Add first end bracket before last comma in string
    let firstcom = new_str.rfind(',').unwrap();
    new_str.insert(firstcom, ')');
    // Give an internal node label after this new bracket
    let mut nstr: String = vec![&new_str[0..=firstcom], &internal_idx.to_string(), &new_str[firstcom+1..new_str.len()]].join("");
    internal_idx += 1;
    // Find last closing bracket in string
    let firstbrack = nstr.rfind(')').unwrap();
    // Add root node label
    nstr = vec![&nstr[0..=firstbrack], &internal_idx.to_string(), &";"].join("");
    // Add corresponding opening bracket to start of string
    nstr.insert(0, '(');

    // This section goes through the Newick string and records the parent nodes of each node
    // so that we can build a Tree struct
    let mut current_parent: Option<usize> = None;
    let mut parent_vector: Vec<Option<usize>> = vec![None; internal_idx + 1];
    let mut idx: Option<usize> = Some(internal_idx);

    for i in (1..nstr.len()).rev() {

        let ch: char = nstr.chars().nth(i).unwrap();

        if ch.eq(&')') || ch.eq(&',') {

            if ch.eq(&')') {
                current_parent = idx;
            }

            let mut j = i - 1;
            let mut jch: char = nstr.chars().nth(j).unwrap();
            while j > 0 && jch.ne(&'(') && jch.ne(&',') && jch.ne(&')') {
                j -= 1;
                jch = nstr.chars().nth(j).unwrap();
            }

            if j != (i - 1) {
                let mut leaf: &str = &nstr[(j + 1)..i];
                idx = Some(leaf.parse().unwrap());
                parent_vector[idx.unwrap()] = current_parent;
            }

        } else if ch.eq(&'(') {
            current_parent = parent_vector[current_parent.unwrap()];
        }
    };

    // Build the tree by going over the vector
    let mut proto_tree: Tree = Tree {
        tree_vec: vec![0],
        nodes: vec![Node::default(); internal_idx + 1],
        max_depth: 0,
        label_dictionary,
        changes: HashMap::new(),
        mutation_lists: Vec::new(),
    };

    // Add nodes to Tree from parent vector, give correct branch length
    for (i, parent) in parent_vector.iter().enumerate().rev() {
        proto_tree.add(i, *parent);

        if let Some(lngth) = branch_length.get(&i) {
            proto_tree.nodes[i].branch_length = *lngth;
        } else {
            proto_tree.nodes[i].branch_length = 0.00001;
        }
    };

    proto_tree.tree_vec = newick_to_vector(&proto_tree.newick(), proto_tree.count_leaves());
    proto_tree.max_depth = proto_tree.max_treedepth();

    proto_tree
}


//////////////////////////////////////////////////
// Build an integer vector from a newick string //
//////////////////////////////////////////////////
pub fn newick_to_vector(nw: &String, n_leaves: usize) -> Vec<usize>{
    let_cxx_string!(nw_cpp = nw);
    let x = ffi::doToVector(nw_cpp, n_leaves as i32, false);
    let y: Vec<usize> = x.iter().map(|el| *el as usize).collect();
    y
}
// Bridging function to C++ code
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("bactrees/include/phylo2vec.hpp");
        fn doToVector(newick: Pin<&mut CxxString>, num_leaves: i32, with_mapping: bool) -> UniquePtr<CxxVector<i32>>;
    }
}



pub fn phylo2vec_lin(v: Vec<usize>, permute: bool) -> Tree {
    let mut tree = Tree::new(&v);
    let mut sub_vec = tree.tree_vec.clone();
    sub_vec.remove(0);
    let k = sub_vec.len();
    let mut M = Array2::<usize>::zeros((k, 3));
    let mut labels_rowk: Vec<usize> = (0..=k).collect();
    let mut rmk = k;

    for i in 0..k {
        let n = k - i - 1;
        let m = sub_vec[n];

        M[[i, 0]] = labels_rowk[m];
        M[[i, 1]] = labels_rowk[n + 1];

        rmk += 1;
        labels_rowk[m] = rmk;
        M[[i, 2]] = labels_rowk[m];
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

    vec![0; k + 1]
        .iter()
        .enumerate()
        .map(|(i, _el)| if i > 0 { rng.gen_range(0..((2 * i) - 1)) } else { 0 })
        .collect()
}

impl Tree {
    pub fn update(&mut self, new_vec: &[usize]) {

        let new_tree: Tree = vector_to_tree(new_vec);
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