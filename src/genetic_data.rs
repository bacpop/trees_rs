use crate::topology::Topology;
use logaddexp::LogAddExp;
use ndarray::s;
use needletail::parse_fastx_file;
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
    let mut n_seqs = 0;
    while let Some(_record) = reader.next() {
        n_seqs += 1;
    }
    n_seqs
}

pub fn create_genetic_data(
    filename: &str,
    topology: &Topology,
    rate_matrix: &na::Matrix4<f64>,
) -> ndarray::Array3<f64> {
    // Count number of sequences and their length
    let mut n_seqs = 0;
    let mut n_bases = 0;
    let mut reader = parse_fastx_file(filename).expect("Error parsing file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("Invalid record");
        n_seqs += 1;
        n_bases = seqrec.num_bases();
    }
    // Create pre-filled array
    let mut gen_data = ndarray::Array3::from_elem((2 * n_seqs - 1, n_bases, 4), -99.0);
    // println!("Assigning data for {} leaves and {} total nodes", n_seqs, (2 * n_seqs) + 1);

    let mut reader2 = parse_fastx_file(filename).expect("Error parsing file");
    let mut seq_i = 0;
    while let Some(record) = reader2.next() {
        let seqrec = record.expect("Invalid record");
        for (loc_i, e) in seqrec.seq().iter().enumerate() {
            let cur = char_to_likelihood(&(*e as char));
            for j in 0..4 {
                gen_data[[seq_i, loc_i, j]] = *cur.get(j).unwrap();
            }
        }
        seq_i += 1;
    }

    create_internal_data(gen_data, topology, rate_matrix)
}

pub fn create_internal_data(
    mut data: ndarray::Array3<f64>,
    topology: &Topology,
    rate_matrix: &na::Matrix4<f64>,
) -> ndarray::Array3<f64> {
    // Iterate over internal nodes postorder
    let nodes = topology.postorder_notips(topology.get_root());

    for node in nodes {
        let i = node.get_id();
        // Calculate node likelihood
        let lchild = node.get_lchild().unwrap();
        let rchild = node.get_rchild().unwrap();
        let node_ll = node_likelihood(
            data.slice(s![lchild, .., ..]),
            data.slice(s![rchild, .., ..]),
            &matrix_exp(rate_matrix, topology.nodes[lchild].get_branchlen()),
            &matrix_exp(rate_matrix, topology.nodes[lchild].get_branchlen()),
        );
        // let node_ll = node_likelihood(node.get_lchild().unwrap(), node.get_rchild().unwrap(), &gen_data, topology, rate_matrix);
        // let node_ll = node_likelihood(i, &gen_data, topology, rate_matrix);
        // Add to genetic data array
        data.slice_mut(s![i, .., ..]).assign(&node_ll);
    }

    data
}

pub fn create_dummy_gendata(
    n_bases: usize,
    topology: &Topology,
    rate_matrix: &na::Matrix4<f64>,
) -> ndarray::Array3<f64> {
    let n_seqs = topology.count_leaves();

    let mut gen_data = ndarray::Array3::from_elem(((2 * n_seqs) + 1, n_bases, 4), 0.0);

    let mut rng = thread_rng();

    for i in 0..n_seqs {
        for j in 0..n_bases {
            let k = rng.gen_range(0..4);
            gen_data[[i, j, k]] = NEGINF;
        }
    }

    create_internal_data(gen_data, topology, rate_matrix)
}

pub fn child_likelihood_i(
    i: usize,
    ll: ndarray::ArrayView1<f64>,
    p: &na::Matrix4<f64>,
) -> f64 {
    p.row(i)
        .iter()
        .zip(ll.iter())
        .map(|(a, b)| a.ln() + *b)
        .reduce(|a, b| a.ln_add_exp(b))
        .unwrap()
}

pub fn matrix_exp(rate_matrix: &na::Matrix4<f64>, branch_len: f64) -> na::Matrix4<f64> {
    na::Matrix::exp(&(rate_matrix * branch_len))
}

pub fn node_likelihood(
    seql: ndarray::ArrayView2<f64>,
    seqr: ndarray::ArrayView2<f64>,
    matrixl: &na::Matrix4<f64>,
    matrixr: &na::Matrix4<f64>,
) -> ndarray::Array2<f64> {
    let out = ndarray::Array2::from_shape_fn((seql.dim().0, 4), |(i, j)| {
        child_likelihood_i(j, seql.slice(s![i, ..]), matrixl)
            + child_likelihood_i(j, seqr.slice(s![i, ..]), matrixr)
    });

    out
}

pub const BF_DEFAULT: [f64; 4] = [0.25, 0.25, 0.25, 0.25];

pub fn base_freq_logse(
    muta: ndarray::ArrayView1<f64>,
    bf: [f64; 4],
) -> f64 {
    muta.iter()
        .zip(bf.iter())
        .fold(0.0, |tot, (muta, bf)| tot + muta.exp() * bf)
        .ln()
}

impl Topology {
    pub fn find_changes(&self, other: &Topology) -> Option<Vec<usize>> {
        let out: Vec<usize> = self
            .nodes
            .iter()
            .zip(other.nodes.iter())
            .filter(|(a, b)| a.get_parent().ne(&b.get_parent()))
            .map(|(a, _)| a.get_id())
            .collect();
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }
}
