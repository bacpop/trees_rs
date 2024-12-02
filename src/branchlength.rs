use crate::{rate_matrix::RateMatrix, treestate::TreeMove};
use rand::prelude::Distribution;
// use rand::distributions::Normal;
use statrs::distribution::Normal;
use crate::topology::Topology;
use crate::TreeState;
pub struct BranchMove{
    indices: Vec<usize>,
}

impl<R: RateMatrix> TreeMove<R> for BranchMove {
    fn generate(&self, ts: &crate::treestate::TreeState<R>) -> crate::treestate::TreeState<R> {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut changes: Vec<usize> = Vec::new();

        // This is not ideal
        let mut nodes = ts.top.nodes.clone();

        for i in self.indices.iter() {
            let ind = *i;
            let mut bl = nodes[ind].get_branchlen();
            bl = bl.ln() + normal.sample(&mut rand::thread_rng());
            nodes[ind].set_branchlen(bl.exp());
            changes.push(ind);
        }

        let new_top = Topology{
            nodes: nodes,
            tree_vec: ts.top.tree_vec.clone(),
            likelihood: ts.top.likelihood,
        };

        TreeState{
            top: new_top,
            mat: ts.mat,
            ll: ts.ll,
            changed_nodes: Some(changes),
        }
    }
}