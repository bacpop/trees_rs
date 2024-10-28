use needletail::parse_fastx_file;
use crate::topology::Topology;
use ndarray::s;
use logaddexp::LogAddExp;

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
        ndarray::Array3::from_elem(((2 * n_seqs) + 1, n_bases, 4), -99.0);
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


pub fn create_internal_data(mut gen_data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>,
                                           topology: &Topology, rate_matrix: &na::Matrix4<f64>) -> 
                                           ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> {
    // Iterate over internal nodes postorder
    let mut nodes = topology.postorder_notips(topology.get_root());

    while let Some(node) = nodes.next() {
        let i = node.get_id();
        // Calculate node likelihood
        let node_ll = node_likelihood(i, &gen_data, topology, rate_matrix);
        // Add to genetic data array
        gen_data.slice_mut(s![i, .., ..]).assign(&node_ll);
    }

    gen_data
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

pub fn node_likelihood(index: usize, 
    gen_data: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>,
    topology: &Topology,
    rate_matrix: &na::Matrix4<f64>) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {

        let lchild = topology.nodes[index].get_lchild().unwrap();
        let rchild = topology.nodes[index].get_rchild().unwrap();
        let bl = topology.nodes[lchild].get_branchlen();
        let br = topology.nodes[rchild].get_branchlen();

        let pl = na::Matrix::exp(&(rate_matrix * bl));
        let pr = na::Matrix::exp(&(rate_matrix * br));

        let seql = gen_data.slice(s![lchild, .., ..]);
        let seqr = gen_data.slice(s![rchild, .., ..]);

        let out = ndarray::Array2::from_shape_fn((seql.dim().0, 4), |(i, j)| 
        child_likelihood_i(j, seql.slice(s![i, ..]), &pl) + 
        child_likelihood_i(j, seqr.slice(s![i, ..]), &pr));

        out
}

const BF_DEFAULT: [f64; 4] = [0.25, 0.25, 0.25, 0.25];

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

// A move produces a vector of nodes that have a new parent node
// Now Likelihood update
// Iterate over nodes that need changing from Topology
// Calculate new likelihoods at internal nodes, add to HashMap
// Get whole Topology likelihood at root node from HashMap
// If better, can move HashMap likelihoods into GeneticData