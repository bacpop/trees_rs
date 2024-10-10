// use argmin::core::{Error, CostFunction, Gradient, Hessian};
// use crate::tree::Tree;
// use crate::rate_matrix::RateMatrix;

// impl<T: RateMatrix> CostFunction for Tree<T> {
//     type Param = Vec<f64>;
//     type Output = f64;

//     fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        
//         self.rate_matrix.update_params(*param);
//         self.initialise_likelihood();
//         return Ok(self.get_tree_likelihood())
//     }
// }