use crate::base_freq_logse;
use crate::newick_to_vec::newick_to_vector;
use crate::BF_DEFAULT;
use ndarray::s;
use ndarray::Array2;
use std::collections::HashMap;

// Node ID, Parent, left child, right child, branchlength,  depth
#[derive(Debug, Clone)]
pub struct NodeTuple(
    pub usize,
    pub Option<usize>,
    pub Option<usize>,
    pub Option<usize>,
    pub f64,
    pub usize,
);

impl NodeTuple {
    pub fn get_id(&self) -> usize {
        self.0
    }

    pub fn get_parent(&self) -> Option<usize> {
        self.1
    }

    pub fn set_parent(&mut self, p: Option<usize>) {
        self.1 = p;
    }

    pub fn get_branchlen(&self) -> f64 {
        self.4
    }

    pub fn set_branchlen(&mut self, b: f64) {
        self.4 = b;
    }

    pub fn get_lchild(&self) -> Option<usize> {
        self.2
    }

    pub fn set_lchild(&mut self, c: Option<usize>) {
        self.2 = c;
    }

    pub fn get_rchild(&self) -> Option<usize> {
        self.3
    }

    pub fn set_rchild(&mut self, c: Option<usize>) {
        self.3 = c;
    }

    pub fn get_depth(&self) -> usize {
        self.5
    }

    pub fn set_depth(&mut self, d: usize) {
        self.5 = d;
    }

    // Add a child to node, depending on children already there
    pub fn add_child(&mut self, c: Option<usize>) {
        match (self.get_lchild(), self.get_rchild()) {
            (None, None) => self.set_lchild(c),
            (Some(_), None) => self.set_rchild(c),
            (None, Some(_)) => self.set_lchild(c),
            (Some(_), Some(_)) => {
                panic!("Trying to add new child to node with 2 children already")
            }
        };
    }
}

impl PartialEq for NodeTuple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug)]
pub struct Topology {
    pub nodes: Vec<NodeTuple>,
    pub tree_vec: Vec<usize>,
}

// Builds a vector of NodeTuples from an integer tree vector
pub fn from_vec(tree_vec: &[usize]) -> Topology {
    let mut sub_vec = tree_vec.to_vec();
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

    // let mut nodes: Vec<NodeTuple> = Vec::with_capacity(2 * k + 1);
    let mut nodes: Vec<NodeTuple> = vec![NodeTuple(0, None, None, None, 0.0, 0); 2 * k + 1];
    nodes[M[[k - 1, 2]]] = NodeTuple(M[[k - 1, 2]], None, None, None, 1.0, 0);

    for i in (0..k).rev() {
        // New depth is parent depth + 1
        let dpth = nodes[M[[i, 2]]].get_depth() + 1;
        // Add new child nodes
        nodes[M[[i, 0]]] = NodeTuple(M[[i, 0]], Some(M[[i, 2]]), None, None, 1.0, dpth);
        nodes[M[[i, 1]]] = NodeTuple(M[[i, 1]], Some(M[[i, 2]]), None, None, 1.0, dpth);
        // Update children in parent node
        nodes[M[[i, 2]]].add_child(Some(M[[i, 0]]));
        nodes[M[[i, 2]]].add_child(Some(M[[i, 1]]));
    }

    Topology {
        nodes,
        tree_vec: tree_vec.to_vec(),
    }
}

// Builds a new Topology from a Newick String
pub fn from_newick(rjstr: String) -> Vec<NodeTuple> {
    let mut new_str: String = rjstr.clone();
    let full_split: Vec<&str> = rjstr
        .split(['(', ',', ')', ';'])
        .filter(|c| !c.is_empty())
        .collect();
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
        new_str = new_str.replace(x, &idx.to_string());
    }

    // This section tries to solve the polytomy at the root of rapidNJ trees
    // By splicing in some brackets and naming more internal nodes
    // Add first end bracket before last comma in string
    let firstcom = new_str.rfind(',').unwrap();
    new_str.insert(firstcom, ')');
    // Give an internal node label after this new bracket
    let mut nstr: String = [
        &new_str[0..=firstcom],
        &internal_idx.to_string(),
        &new_str[firstcom + 1..new_str.len()],
    ]
    .join("");
    internal_idx += 1;
    // Find last closing bracket in string
    let firstbrack = nstr.rfind(')').unwrap();
    // Add root node label
    nstr = [&nstr[0..=firstbrack], &internal_idx.to_string(), &";"].join("");
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
                let leaf: &str = &nstr[(j + 1)..i];
                idx = Some(leaf.parse().unwrap());
                parent_vector[idx.unwrap()] = current_parent;
            }
        } else if ch.eq(&'(') {
            current_parent = parent_vector[current_parent.unwrap()];
        }
    }

    // Add nodes to Tree from parent vector, give correct branch length
    let mut nodevec: Vec<NodeTuple> =
        vec![NodeTuple(0, None, None, None, 0.0, 0); internal_idx + 1];

    for (i, parent) in parent_vector.iter().enumerate().rev() {
        let mut dpth = 0;
        if let Some(par) = parent {
            // Add this child to existing parent
            nodevec[*par].add_child(Some(i));
            // Get child depth from parent + 1
            dpth = nodevec[*par].get_depth() + 1;
        }
        // Update this child NodeTuple
        nodevec[i].set_parent(*parent);
        nodevec[i].set_depth(dpth);
    }

    nodevec
}

impl Topology {
    // Builds a Newick String for a Topology object
    pub fn get_newick(&self) -> String {
        let mut current_node: Option<&NodeTuple> = Some(self.get_root());
        let mut next_node: Option<&NodeTuple>;
        let mut return_nodes: Vec<Option<&NodeTuple>> = Vec::new();
        let mut newick: Vec<String> = vec![
            String::from(";"),
            current_node.unwrap().get_branchlen().to_string(),
            String::from(":"),
            current_node.unwrap().get_id().to_string(),
        ];

        while current_node.is_some() {
            match (
                current_node.unwrap().get_lchild(),
                current_node.unwrap().get_rchild(),
            ) {
                (Some(a), None) => {
                    next_node = self.nodes.get(a);

                    newick.push(String::from(")"));
                    newick.push(next_node.unwrap().get_branchlen().to_string());
                    newick.push(String::from(":"));
                    newick.push(next_node.unwrap().get_id().to_string());
                }
                (Some(a), Some(b)) => {
                    next_node = self.nodes.get(a);

                    return_nodes.push(self.nodes.get(b));

                    newick.push(String::from(")"));
                    newick.push(next_node.unwrap().get_branchlen().to_string());
                    newick.push(String::from(":"));
                    newick.push(next_node.unwrap().get_id().to_string());
                }
                (None, _) => {
                    next_node = match return_nodes.pop() {
                        None => None,
                        Some(a) => a,
                    };
                    if next_node.is_some() {
                        let n: usize =
                            current_node.unwrap().get_depth() - next_node.unwrap().get_depth();
                        match n {
                            0 => {
                                newick.push(String::from(","));
                            }
                            _ => {
                                for _ in 1..=n {
                                    newick.push(String::from("("));
                                }
                                newick.push(String::from(","));
                            }
                        }

                        newick.push(next_node.unwrap().get_branchlen().to_string());
                        newick.push(String::from(":"));
                        newick.push(next_node.unwrap().get_id().to_string());
                    } else {
                        let n: usize = current_node.unwrap().get_depth();
                        for _ in 1..=n {
                            newick.push(String::from("("));
                        }
                    }
                }
            }
            current_node = next_node;
        }

        newick.reverse();
        newick.concat()
    }

    pub fn count_leaves(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| n.get_lchild().is_none() && n.get_rchild().is_none())
            .count()
    }

    pub fn get_lchild(&self, node: &NodeTuple) -> Option<&NodeTuple> {
        if let Some(index) = node.get_lchild() {
            Some(&self.nodes[index])
        } else {
            None
        }
    }

    pub fn get_rchild(&self, node: &NodeTuple) -> Option<&NodeTuple> {
        if let Some(index) = node.get_rchild() {
            Some(&self.nodes[index])
        } else {
            None
        }
    }

    pub fn get_parent(&self, node: &NodeTuple) -> Option<&NodeTuple> {
        if let Some(index) = node.get_parent() {
            Some(&self.nodes[index])
        } else {
            None
        }
    }

    pub fn get_root(&self) -> &NodeTuple {
        self.nodes
            .iter()
            .find(|node| node.get_parent().is_none())
            .unwrap()
    }

    pub fn likelihood(
        &self,
        gen_data: &ndarray::Array3<f64>,
    ) -> f64 {
        let root_likelihood = gen_data.slice(s![self.get_root().get_id(), .., ..]);

        root_likelihood
            .rows()
            .into_iter()
            .fold(0.0, |acc, base| acc + base_freq_logse(base, BF_DEFAULT))
    }
}
