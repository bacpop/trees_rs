use crate::topology::Topology;
use rand::{thread_rng, Rng};
use crate::genetic_data::{node_likelihood, slice_data, matrix_exp};
use ndarray::s;
use logaddexp::LogAddExp;

const NEGINF: f64 = -f64::INFINITY;

pub fn create_dummy_statedata(n_mges: usize, topology: &Topology, rate_matrix: &na::Matrix2<f64>) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> {
    let n_seqs = topology.count_leaves();

    let mut state_data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> = 
        ndarray::Array3::from_elem(((2 * n_seqs) - 1, n_mges, 2), 0.0);

    let mut rng = thread_rng();

    for i in 0..n_seqs {
        for j in 0..n_mges {
            let k = rng.gen_range(0..2);
                state_data[[i, j, k]] = NEGINF;
        }
    }
    
    // create internal state_data
    create_internal_statedata(state_data, &topology, rate_matrix)
}

pub fn create_internal_statedata(mut data: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>,
    topology: &Topology, rate_matrix: &na::Matrix2<f64>) -> 
    ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>> {
    // Iterate over internal nodes postorder
    let mut nodes = topology.postorder_notips(topology.get_root());

    while let Some(node) = nodes.next() {
    let i = node.get_id();
    // Calculate node likelihood
    let lchild = node.get_lchild().unwrap();
    let rchild = node.get_rchild().unwrap();
    let node_ll = state_likelihood(slice_data(lchild, &data),
    slice_data(rchild, &data), 
    &matrix_exp2(rate_matrix, topology.nodes[lchild].get_branchlen()),
    &matrix_exp2(rate_matrix, topology.nodes[lchild].get_branchlen()));

    // Add to genetic data array
    data.slice_mut(s![i, .., ..]).assign(&node_ll);
    }

    data
}


pub fn matrix_exp2(rate_matrix: &na::Matrix2<f64>, branch_len: f64) -> na::Matrix2<f64> {
    na::Matrix::exp(&(rate_matrix * branch_len))
}

pub fn state_likelihood(seql: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>,
    seqr: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>,
    matrixl: &na::Matrix2<f64>,
    matrixr: &na::Matrix2<f64>,
    ) -> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {

        let out = ndarray::Array2::from_shape_fn((seql.dim().0, 2), |(i, j)| 
        state_likelihood_i(j, seql.slice(s![i, ..]), &matrixl) + 
        state_likelihood_i(j, seqr.slice(s![i, ..]), &matrixr));

        out
}

pub fn state_likelihood_i(i: usize, ll: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 1]>>, p: &na::Matrix2<f64>) -> f64 {
    p
            .row(i)
            .iter()
            .zip(ll.iter())
            .map(|(a, b)| a.ln() + *b)
            .reduce(|a, b| a.ln_add_exp(b))
            .unwrap()
}