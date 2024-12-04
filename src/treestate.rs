use crate::topology::NodeTuple;
use crate::Topology;
use crate::RateMatrix;
use crate::{base_freq_logse, matrix_exp, slice_data, node_likelihood, BF_DEFAULT};
use std::collections::HashMap;
use crate::ExactMove;
use ndarray::s;

pub struct TreeState<R: RateMatrix>{
    pub top: Topology,
    pub mat: R,
    pub ll: Option<f64>,
    pub changed_nodes: Option<Vec<usize>>,
}

pub trait TreeMove<R: RateMatrix> {
    fn generate(&self, ts: &TreeState<R>) -> TreeState<R>;
}

impl<R: RateMatrix> TreeState<R> {

    pub fn likelihood(&self, gen_data: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>) -> f64 {
        let root_likelihood = gen_data.slice(s![self.top.get_root().get_id(), .., .. ]);

        root_likelihood
        .rows()
        .into_iter()
        .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT))
    }


//     pub fn apply_move<T: TreeMove<R>>(&mut self, 
//         move_fn: T,
//         accept_fn: fn(&f64, &f64) -> bool, 
//         gen_data: &mut ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>) -> TreeState<R> {

//         if self.ll.is_none() {
//             self.ll = Some(self.likelihood(gen_data));
//         }
//         let old_ll = self.ll.unwrap();

//         let rate_mat = self.mat.get_matrix();
//         let new_ts = move_fn.generate(self);

//         // If move did nothing, keep old TreeState
//         if new_ts.changed_nodes.is_none() {
//             return *self
//         }

//         // Do minimal likelihood updates (and push new values into HashMap temporarily)
//         let nodes = new_ts.top.changes_iter_notips(new_ts.changed_nodes.unwrap());
//         let mut temp_likelihoods: HashMap<usize, ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>> = HashMap::new();

//         for node in nodes {
//             // check if in HM
//             // println!("{:?}", node);
//             let lchild = node.get_lchild().unwrap();
//             let rchild = node.get_rchild().unwrap();
//             let seql: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;
//             let seqr: ndarray::ArrayBase<ndarray::ViewRepr<&f64>, ndarray::Dim<[usize; 2]>>;

//             match (temp_likelihoods.contains_key(&lchild), temp_likelihoods.contains_key(&rchild)) {
//                 (true, true) => {
//                     seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
//                     seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
//                 },
//                 (true, false) => {
//                     seql = temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..]);
//                     seqr = slice_data(rchild, gen_data);
//                 },
//                 (false, true) => {
//                     seql = slice_data(lchild, gen_data);
//                     seqr = temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..]);
//                 },
//                 (false, false) => {
//                     seql = slice_data(lchild, gen_data);
//                     seqr = slice_data(rchild, gen_data);
//                 },
//             };

//             let node_ll = node_likelihood(seql, seqr, 
//                 &matrix_exp(&rate_mat, new_ts.top.nodes[lchild].get_branchlen()),
//                 &matrix_exp(&rate_mat, new_ts.top.nodes[rchild].get_branchlen()));

//             temp_likelihoods.insert(node.get_id(), node_ll);
//         }

//         // Calculate whole new topology likelihood at root
//     let new_ll = temp_likelihoods
//     .get(&new_ts.top.get_root().get_id())
//     .unwrap()
//     .rows()
//     .into_iter()
//     .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT));

//     // Likelihood decision rule
//     if accept_fn(&old_ll, &new_ll) {
//         // Drain hashmap into gen_data
//         for (i, ll_data) in temp_likelihoods.drain() {
//             gen_data.slice_mut(s![i, .., ..]).assign(&ll_data);
//         }
//         // Update Topology
//         self.top.nodes = new_ts.top.nodes;
//         self.top.tree_vec = new_ts.top.tree_vec;
//         self.mat = new_ts.mat;
//         self.ll = Some(new_ll);
//     };
//     *self
// }
}

pub fn hillclimb_accept(old_ll: &f64, new_ll: &f64) -> bool {
    new_ll.gt(old_ll)
}

pub fn always_accept(_old_ll: &f64, _new_ll: &f64) -> bool {
    true
}


pub struct TreeStateIter<'a, R: RateMatrix, M: TreeMove<R>> {
    pub ts: TreeState<R>,
    pub move_fn: M,
    pub accept_fn: fn(&f64, &f64) -> bool,
    pub gen_data: &'a mut ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 3]>>,
}


impl<'a, R: RateMatrix + 'a, M: TreeMove<R>> Iterator for TreeStateIter<'a, R, M> {
    type Item = TreeState<R>;
    fn next(&mut self) -> Option<Self::Item> {

        if self.ts.ll.is_none() {
            self.ts.ll = Some(self.ts.likelihood(self.gen_data));
        }
        let old_ll = self.ts.ll.unwrap();

        let mut new_ts = self.move_fn.generate(&self.ts);
        let rate_mat = new_ts.mat.get_matrix();

        if new_ts.changed_nodes.is_none() {
            return Some(new_ts)
        }

        let changed_nodes = new_ts.changed_nodes.clone().unwrap();
        let nodes = new_ts.top.changes_iter_notips(changed_nodes);
        let mut temp_likelihoods: HashMap<usize, ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>> = HashMap::new();

        for node in nodes {
            let (lchild, rchild) = (node.get_lchild().unwrap(), node.get_rchild().unwrap());

            let seql = match temp_likelihoods.contains_key(&lchild) {
                true => {temp_likelihoods.get(&lchild).unwrap().slice(s![.., ..])},
                false => {slice_data(lchild, self.gen_data)},
            };
            let seqr = match temp_likelihoods.contains_key(&rchild) {
                true => {temp_likelihoods.get(&rchild).unwrap().slice(s![.., ..])},
                false => {slice_data(rchild, self.gen_data)},
            };

            let node_ll = node_likelihood(seql, seqr, 
                &matrix_exp(&rate_mat, new_ts.top.nodes[lchild].get_branchlen()),
                &matrix_exp(&rate_mat, new_ts.top.nodes[rchild].get_branchlen()));

            temp_likelihoods.insert(node.get_id(), node_ll);
        }

        // Calculate whole new topology likelihood at root
        let new_ll = temp_likelihoods
        .get(&new_ts.top.get_root().get_id())
        .unwrap()
        .rows()
        .into_iter()
        .fold(0.0, |acc, base | acc + base_freq_logse(base, BF_DEFAULT));

        if (self.accept_fn)(&old_ll, &new_ll) {
            // Drain hashmap into gen_data
            for (i, ll_data) in temp_likelihoods.drain() {
                self.gen_data.slice_mut(s![i, .., ..]).assign(&ll_data);
            }
            new_ts.ll = Some(new_ll);
            // Return new TreeState
            return Some(new_ts)
        } else {
            // Return old TreeState
            let top = Topology{
                nodes: self.ts.top.nodes.clone(),
                tree_vec: self.ts.top.tree_vec.clone(),
                likelihood: None,
            };

            return Some(TreeState{
                top,
                mat: self.ts.mat,
                ll: Some(old_ll),
                changed_nodes: None,
            })
        }
    }
}

// impl<'a, R: RateMatrix, M: TreeMove<R>> TreeState<R> {
//     pub fn moveiter() -> TreeStateIter<'a, R, M> {
//         todo!()
//     }
// }