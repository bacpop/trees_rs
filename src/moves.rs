use std::env::current_exe;

use crate::newick_to_vector;
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

pub struct NearestNeighbour{

}

impl MoveFn for NearestNeighbour {
    fn generate_move(&self, current_topology: &Topology) -> CandidateTopology {
        
        let mut new_topology: Topology = Topology{
            nodes: current_topology.nodes.clone(),
            tree_vec: current_topology.tree_vec.clone(),
            likelihood: None,
        };

        // Get vector of all internal nodes that can be swapped
        let mut int_nodes: Vec<usize> = current_topology
        .postorder_notips(current_topology.get_root())
        .map(|node| node.get_id()).collect();

        // Last element is root, which we can't swap with parent so pop it off
        int_nodes.pop();

        let ind = rand::thread_rng().gen_range(0..int_nodes.len());
        let child_index = int_nodes[ind];

        // Children and parent of selected node
        let (lc, rc) = (current_topology.nodes[child_index].get_lchild(), current_topology.nodes[child_index].get_rchild()); 
        let child_node_parent = current_topology.nodes[child_index].get_parent().unwrap();
        
        // Children and parent of selected node's parent
        let (lp, rp) = (current_topology.nodes[child_node_parent].get_lchild(), current_topology.nodes[child_node_parent].get_rchild());
        let parent_node_parent = current_topology.nodes[child_node_parent].get_parent().unwrap();
        
        // Assign new parent and children to selected node's parent
        new_topology.nodes[child_node_parent].set_lchild(lc);
        new_topology.nodes[child_node_parent].set_rchild(rc);

        let (lpc, rpc) = (current_topology.nodes[parent_node_parent].get_lchild(), current_topology.nodes[parent_node_parent].get_rchild());
        if lpc == Some(child_node_parent) {
            new_topology.nodes[parent_node_parent].set_rchild(rpc);
        } else {
            new_topology.nodes[parent_node_parent].set_lchild(lpc);
        }

        new_topology.nodes[child_node_parent].set_parent(Some(child_index));
        new_topology.nodes[lc.unwrap()].set_parent(Some(child_node_parent));
        new_topology.nodes[rc.unwrap()].set_parent(Some(child_node_parent));

        new_topology.nodes[child_index].set_parent(Some(parent_node_parent));

        if lp == Some(child_index) {
            new_topology.nodes[child_index].set_rchild(rp);
            new_topology.nodes[rp.unwrap()].set_parent(Some(child_index));
        } else {
            new_topology.nodes[child_index].set_lchild(lp);
            new_topology.nodes[lp.unwrap()].set_parent(Some(child_index));
        }

        new_topology.tree_vec = newick_to_vector(&new_topology.get_newick(), new_topology.count_leaves());
        
        CandidateTopology{
            new_topology,
            changes: Some(vec![child_node_parent]),
        }
    }
}


pub fn hillclimb_accept(old_ll: &f64, new_ll: &f64) -> bool {
    new_ll.gt(old_ll)
}

pub fn always_accept(_old_ll: &f64, _new_ll: &f64) -> bool {
    true
}