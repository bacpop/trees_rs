use crate::RateMatrix;
use crate::Topology;
use crate::TreeMove;
use crate::{base_freq_logse, matrix_exp, node_likelihood, BF_DEFAULT};
use std::collections::HashMap;
use ndarray::s;

pub struct TreeState<R: RateMatrix> {
    pub top: Topology,
    pub mat: R,
    pub likelihood: f64,
}

pub fn apply_move<M: TreeMove<R>, R: RateMatrix>(
    current_ts: TreeState<R>,
    move_fn: M,
    accept_fn: fn(&f64, &f64) -> bool,
    gen_data: &mut ndarray::Array3<f64>,
) -> TreeState<R> {
    let (new_topology, new_mat, changes) = move_fn.generate(&current_ts);

    if changes.is_none() {
        return current_ts;
    }

    let rate_matrix = match new_mat {
        Some(x) => x.get_matrix(),
        None => current_ts.mat.get_matrix(),
    };

    let candidate_top = match new_topology {
        Some(t) => t,
        None => current_ts.top,
    };

    let nodes_to_update = candidate_top.changes_iter_notips(changes.unwrap());

    let mut temp_likelihoods: HashMap<
        usize,
        ndarray::Array2<f64>,
    > = HashMap::new();

    for node in nodes_to_update {
        let (lchild, rchild) = (node.get_lchild().unwrap(), node.get_rchild().unwrap());

        let seql = match temp_likelihoods.contains_key(&lchild) {
            true => temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]),
            false => gen_data.slice(s![lchild, .., ..]),
        };
        let seqr = match temp_likelihoods.contains_key(&rchild) {
            true => temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]),
            false => gen_data.slice(s![rchild, .., ..]),
        };

        let node_ll = node_likelihood(
            seql,
            seqr,
            &matrix_exp(&rate_matrix, candidate_top.nodes[lchild].get_branchlen()),
            &matrix_exp(&rate_matrix, candidate_top.nodes[rchild].get_branchlen()),
        );

        temp_likelihoods.insert(node.get_id(), node_ll);
    }

    // Calculate whole new topology likelihood at root
    let new_ll = temp_likelihoods
        .get(&candidate_top.get_root().get_id())
        .unwrap()
        .rows()
        .into_iter()
        .fold(0.0, |acc, base| acc + base_freq_logse(base, BF_DEFAULT));

    if accept_fn(&current_ts.likelihood, &new_ll) {
        // Drain hashmap into gen_data
        for (i, ll_data) in temp_likelihoods.drain() {
            gen_data.slice_mut(s![i, .., ..]).assign(&ll_data);
        }
        let nm = match new_mat {
            Some(x) => x,
            None => current_ts.mat,
        };
        TreeState {
            top: candidate_top,
            mat: nm,
            likelihood: new_ll,
        }
    } else {
        TreeState {
            top: candidate_top,
            mat: current_ts.mat,
            likelihood: current_ts.likelihood,
        }
    }
}

pub fn hillclimb_accept(old_ll: &f64, new_ll: &f64) -> bool {
    new_ll.gt(old_ll)
}

pub fn always_accept(_old_ll: &f64, _new_ll: &f64) -> bool {
    true
}
