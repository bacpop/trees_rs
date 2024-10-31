use crate::Topology;
use crate::CandidateTopology;
use rand::Rng;

pub trait MoveFn {
    fn generate_move(&self, current_topology: &Topology) -> CandidateTopology;
}

pub struct ExactMove {
    pub target_vector: Vec<usize>,
}

impl MoveFn for ExactMove {
    fn generate_move(&self, current_topology: &Topology) -> CandidateTopology {
        
        let new_topology: Topology = Topology::from_vec(&self.target_vector);
        let changes: Option<Vec<usize>> = current_topology.find_changes(&new_topology);
        CandidateTopology{
            new_topology,
            changes,
        }
    }
}

pub struct PeturbVec {
    pub n: usize,
}

impl MoveFn for PeturbVec {
    fn generate_move(&self, current_topology: &Topology) -> CandidateTopology {
        let mut vout = current_topology.tree_vec.to_vec();
        let mut rng = rand::thread_rng();
        let ind_rng = rand::thread_rng();
        let distr = rand::distributions::Bernoulli::new(0.5).unwrap();
        let ind_distr = rand::distributions::Uniform::new(0, vout.len());

        let samp_n: usize = match self.n.gt(&vout.len()) {
            true => {vout.len()},
            false => {self.n},
        };

        let mut inds: Vec<usize> = ind_rng.sample_iter(ind_distr).take(samp_n).collect();
        inds.sort();

        for ind in inds {
            if ind.eq(&0) {
                continue;
            }
    
            match rng.sample(distr) {
                true => {
                    if vout[ind].lt(&(2 * (ind - 1))) {
                        vout[ind] += 1;
                    }
                }
                false => {
                    if vout[ind].gt(&0) {
                        vout[ind] -= 1;
                    }
                }
            };
        };

        let new_topology: Topology = Topology::from_vec(&vout);
        let changes: Option<Vec<usize>> = current_topology.find_changes(&new_topology);
        CandidateTopology{
            new_topology,
            changes,
        }
    }
}


pub fn hillclimb_accept(old_ll: &f64, new_ll: &f64) -> bool {
    new_ll.gt(old_ll)
}
