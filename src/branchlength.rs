use crate::{rate_matrix::RateMatrix, treestate::TreeMove};

pub struct BranchMove{}

impl<R: RateMatrix> TreeMove<R> for BranchMove {
    fn generate(&self, ts: &crate::treestate::TreeState<R>) -> crate::treestate::TreeState<R> {
        todo!()
    }
}