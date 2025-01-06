use cxx::let_cxx_string;
use rand::Rng;

//////////////////////////////////////////////////
// Build an integer vector from a newick string //
//////////////////////////////////////////////////
pub fn newick_to_vector(nw: &String, n_leaves: usize) -> Vec<usize> {
    let_cxx_string!(nw_cpp = nw);
    let x = ffi::doToVector(nw_cpp, n_leaves as i32, false);
    let y: Vec<usize> = x.iter().map(|el| *el as usize).collect();
    y
}
// Bridging function to C++ code
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("bactrees/include/phylo2vec.hpp");
        fn doToVector(
            newick: Pin<&mut CxxString>,
            num_leaves: i32,
            with_mapping: bool,
        ) -> UniquePtr<CxxVector<i32>>;
    }
}

////////////////////////////////////////////////
// Create a random vector with a given length //
////////////////////////////////////////////////
pub fn random_vector(n_seqs: usize) -> Vec<usize> {
    (0..n_seqs).map(|i| {
        if i > 0 {
            rand::thread_rng().gen_range(0..((2 * i) - 1))
        } else {
            0
        }
    })
    .collect()
}
