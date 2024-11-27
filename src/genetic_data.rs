use std::os::unix::thread;
use std::thread::current;
use std::collections::HashMap;
use needletail::parse_fastx_file;
use crate::topology::Topology;
use ndarray::s;
use logaddexp::LogAddExp;
use crate::moves::MoveFn;
use rand::{thread_rng, Rng};

const NEGINF: f64 = -f64::INFINITY;
// (A, C, G, T)
const AMUT: [f64; 4] = [0.0, NEGINF, NEGINF, NEGINF];
const CMUT: [f64; 4] = [NEGINF, 0.0, NEGINF, NEGINF];
const GMUT: [f64; 4] = [NEGINF, NEGINF, 0.0, NEGINF];
const TMUT: [f64; 4] = [NEGINF, NEGINF, NEGINF, 0.0];
const YMUT: [f64; 4] = [NEGINF, 0.0, NEGINF, 0.0];
const WMUT: [f64; 4] = [0.0, NEGINF, NEGINF, 0.0];
const RMUT: [f64; 4] = [0.0, NEGINF, 0.0, NEGINF];
const KMUT: [f64; 4] = [NEGINF, NEGINF, 0.0, 0.0];
const SMUT: [f64; 4] = [NEGINF, 0.0, 0.0, NEGINF];
const MMUT: [f64; 4] = [0.0, 0.0, NEGINF, NEGINF];
const BMUT: [f64; 4] = [NEGINF, 0.0, 0.0, 0.0];
const DMUT: [f64; 4] = [0.0, NEGINF, 0.0, 0.0];
const VMUT: [f64; 4] = [0.0, 0.0, 0.0, NEGINF];

pub fn char_to_likelihood(e: &char) -> [f64; 4] {
    match e {
        'A' => AMUT,
        'C' => CMUT,
        'G' => GMUT,
        'T' => TMUT,
        'Y' => YMUT,
        'W' => WMUT,
        'R' => RMUT,
        'K' => KMUT,
        'S' => SMUT,
        'M' => MMUT,
        'B' => BMUT,
        'D' => DMUT,
        'V' => VMUT,
        '-' => {
            // This way of coding gives same answer as other tree programs
            [0.0, 0.0, 0.0, 0.0]
        }
        _ => panic!("Unrecognised character: {}", e),
    }
}

pub fn count_sequences(filename: &str) -> usize {
    let mut reader = parse_fastx_file(filename).expect("Error parsing file");
    let mut n_seqs: usize = 0;
    while let Some(_record) = reader.next() {
        n_seqs += 1;
    };
    n_seqs
}


pub fn create_genetic_data(filename: &str, topology: &Topology, rate_matrix: &na::Matrix4<f64>) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> {
    // Count number of sequences and their length
    let mut n_seqs = 0;
    let mut n_bases= 0;
    let mut reader = parse_fastx_file(filename).expect("Error parsing file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("Invalid record");
        n_seqs += 1;
        n_bases = seqrec.num_bases();
    }
    // Create pre-filled array
    let mut gen_data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> = 
        ndarray::Array3::from_elem(((2 * n_seqs) - 1, n_bases, 4), -99.0);
        // println!("Assigning data for {} leaves and {} total nodes", n_seqs, (2 * n_seqs) + 1);

    let mut reader2 = parse_fastx_file(filename).expect("Error parsing file");
    let mut seq_i = 0;
    while let Some(record) = reader2.next() {
        let seqrec = record.expect("Invalid record");
        let mut loc_i = 0;
        for e in seqrec.seq().iter() {
            let cur = char_to_likelihood(&(*e as char));
            for j in 0..4 {
                gen_data[[seq_i, loc_i, j]] = *cur.get(j).unwrap();
            }
            loc_i += 1;
        }                
        seq_i += 1;
    }

    create_internal_data(gen_data, topology, rate_matrix)
}


pub fn create_internal_data(mut data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>,
                                           topology: &Topology, rate_matrix: &na::Matrix4<f64>) -> 
                                           ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> {
    // Iterate over internal nodes postorder
    let mut nodes = topology.postorder_notips(topology.get_root());

    while let Some(node) = nodes.next() {
        let i = node.get_id();
        // Calculate node likelihood
        let lchild = node.get_lchild().unwrap();
        let rchild = node.get_rchild().unwrap();
        let node_ll = node_likelihood(slice_data(lchild, &data),
        slice_data(rchild, &data), 
        &matrix_exp(rate_matrix, topology.nodes[lchild].get_branchlen()),
         &matrix_exp(rate_matrix, topology.nodes[lchild].get_branchlen()));
        // let node_ll = node_likelihood(node.get_lchild().unwrap(), node.get_rchild().unwrap(), &gen_data, topology, rate_matrix);
        // let node_ll = node_likelihood(i, &gen_data, topology, rate_matrix);
        // Add to genetic data array
        data.slice_mut(s![i, .., ..]).assign(&node_ll);
    }

    data
}

pub fn create_dummy_gendata(n_bases: usize, topology: &Topology, rate_matrix: &na::Matrix4<f64>) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> {
    
    let n_seqs = topology.count_leaves();

    let mut gen_data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> = 
        ndarray::Array3::from_elem(((2 * n_seqs) + 1, n_bases, 4), 0.0);
    
    let mut rng = thread_rng();

    for i in 0..n_seqs {
        for j in 0..n_bases {
            let k = rng.gen_range(0..4);
            gen_data[[i, j, k]] = NEGINF;
        }
    }

    create_internal_data(gen_data, topology, rate_matrix)
}
                                    
pub fn child_likelihood_i(i: usize, ll: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 1]>>, p: &na::Matrix4<f64>) -> f64 {
    p
            .row(i)
            .iter()
            .zip(ll.iter())
            .map(|(a, b)| a.ln() + *b)
            .reduce(|a, b| a.ln_add_exp(b))
            .unwrap()
}

pub fn matrix_exp(rate_matrix: &na::Matrix4<f64>, branch_len: f64) -> na::Matrix4<f64> {
    na::Matrix::exp(&(rate_matrix * branch_len))
}

pub fn slice_data(index: usize, data: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>) -> 
ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>> {
        data.slice(s![index, .., ..])
}

pub fn node_likelihood(seql: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>,
    seqr: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>,
    matrixl: &na::Matrix4<f64>,
    matrixr: &na::Matrix4<f64>,
    ) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {

        let out = ndarray::Array2::from_shape_fn((seql.dim().0, 4), |(i, j)| 
        child_likelihood_i(j, seql.slice(s![i, ..]), &matrixl) + 
        child_likelihood_i(j, seqr.slice(s![i, ..]), &matrixr));

        out
}

pub const BF_DEFAULT: [f64; 4] = [0.25, 0.25, 0.25, 0.25];

pub fn likelihood(top: &Topology, gen_data: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>) -> f64 {
    let root_likelihood = gen_data.slice(s![top.get_root().get_id(), .., .. ]);

    root_likelihood
    .rows()
    .into_iter()
    .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT))
}

pub fn base_freq_logse(muta: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 1]>>, bf: [f64; 4]) -> f64 {
    muta.iter()
        .zip(bf.iter())
        .fold(0.0, |tot, (muta, bf)| tot + muta.exp() * bf)
        .ln()
}
pub struct CandidateTopology{
    pub new_topology: Topology,
    pub changes: Option<Vec<usize>>,
}

impl Topology {
    pub fn find_changes(&self, other: &Topology) -> Option<Vec<usize>> {
        let out: Vec<usize> = self.nodes
        .iter()
        .zip(other.nodes.iter())
        .filter(|(a, b)| a.get_parent().ne(&b.get_parent()))
        .map(|(a, b)| a.get_id())
        .collect();
        if out.len() > 0 {
            return Some(out);
        } else {
            return None;
        }
    }

    pub fn apply_move<T: MoveFn>(&mut self, 
        move_fn: T,
        accept_fn: fn(&f64, &f64) -> bool, 
        gen_data: &mut ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>, 
        rate_matrix: &na::Matrix4<f64>) -> () {

        // Get current likelihood, calculating if needed
        if self.likelihood.is_none() {
            self.likelihood = Some(likelihood(&self, gen_data));
        }
        let old_ll = self.likelihood.unwrap();


        // Generate new candidate Topology using move function
        let new_top = move_fn.generate_move(&self);

        println!("Changes: {:?}", new_top.changes);

        // Move did nothing, no changes needed
        if new_top.changes.is_none() {
            return ()
        }

        // Do minimal likelihood updates (and push new values into HashMap temporarily)
        let nodes = new_top.new_topology.changes_iter_notips(new_top.changes.unwrap());
        let mut temp_likelihoods: HashMap<usize, ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>> = HashMap::new();
        for node in nodes {
            // check if in HM
            let lchild = node.get_lchild().unwrap();
            let rchild = node.get_rchild().unwrap();
            let seql: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;
            let seqr: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;

            match (temp_likelihoods.contains_key(&lchild), temp_likelihoods.contains_key(&rchild)) {
                (true, true) => {
                    seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
                    seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
                },
                (true, false) => {
                    seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
                    seqr = slice_data(rchild, &gen_data);
                },
                (false, true) => {
                    seql = slice_data(lchild, &gen_data);
                    seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
                },
                (false, false) => {
                    seql = slice_data(lchild, &gen_data);
                    seqr = slice_data(rchild, &gen_data);
                },
            };

            let node_ll = node_likelihood(seql, seqr, 
                &matrix_exp(rate_matrix, new_top.new_topology.nodes[lchild].get_branchlen()),
                &matrix_exp(rate_matrix, new_top.new_topology.nodes[rchild].get_branchlen()));

            temp_likelihoods.insert(node.get_id(), node_ll);
        }

        // Calculate whole new topology likelihood at root
        let new_ll = temp_likelihoods
        .get(&new_top.new_topology.get_root().get_id())
        .unwrap()
        .rows()
        .into_iter()
        .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT));

        // Likelihood decision rule
        if accept_fn(&old_ll, &new_ll) {
            // Drain hashmap into gen_data
            for (i, ll_data) in temp_likelihoods.drain() {
                gen_data.slice_mut(s![i, .., ..]).assign(&ll_data);
            }
            // Update Topology
            self.nodes = new_top.new_topology.nodes;
            self.tree_vec = new_top.new_topology.tree_vec;
            self.likelihood = Some(new_ll);
        }
    }
}

