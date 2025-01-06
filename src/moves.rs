use crate::newick_to_vector;
// use crate::treestate::TreeMove;
use crate::topology::from_vec;
use crate::RateMatrix;
use crate::Topology;
use crate::TreeState;
use rand::Rng;

pub struct ExactMove {
    pub target_vector: Vec<usize>,
}

pub trait TreeMove<R: RateMatrix> {
    fn generate(
        &self,
        current_treestate: &TreeState<R>,
    ) -> (Option<Topology>, Option<R>, Option<Vec<usize>>);
}

impl<R: RateMatrix> TreeMove<R> for ExactMove {
    fn generate(
        &self,
        current_treestate: &TreeState<R>,
    ) -> (Option<Topology>, Option<R>, Option<Vec<usize>>) {
        let new_topology = from_vec(&self.target_vector);
        let changes = current_treestate.top.find_changes(&new_topology);
        (Some(new_topology), None, changes)
    }
}

pub struct PeturbVec {
    pub n: usize,
}

impl<R: RateMatrix> TreeMove<R> for PeturbVec {
    fn generate(&self, ts: &TreeState<R>) -> (Option<Topology>, Option<R>, Option<Vec<usize>>) {
        let mut vout = ts.top.tree_vec.to_vec();

        let mut rng = rand::thread_rng();
        let ind_rng = rand::thread_rng();
        let distr = rand::distributions::Bernoulli::new(0.5).unwrap();
        let ind_distr = rand::distributions::Uniform::new(0, vout.len());

        let samp_n = match self.n.gt(&vout.len()) {
            true => vout.len(),
            false => self.n,
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
        }

        let new_topology = from_vec(&vout);
        let changes = ts.top.find_changes(&new_topology);
        (Some(new_topology), None, changes)
    }
}

pub struct ChildSwap {}

impl<R: RateMatrix> TreeMove<R> for ChildSwap {
    fn generate(&self, ts: &TreeState<R>) -> (Option<Topology>, Option<R>, Option<Vec<usize>>) {
        // Create new topology
        let mut new_topology = Topology {
            nodes: ts.top.nodes.clone(),
            tree_vec: ts.top.tree_vec.clone(),
        };

        // Select indices of internal nodes
        let mut int_nodes: Vec<usize> = ts
            .top
            .postorder_notips(ts.top.get_root())
            .map(|n| n.get_id())
            .collect();
        // Pop off root
        int_nodes.pop();
        // Randomly choose an internal node
        let ind = int_nodes.remove(rand::thread_rng().gen_range(0..int_nodes.len()));
        // Get index of node and its parent
        let node = ts.top.nodes[ind].get_id();
        let parent = ts.top.get_parent(&ts.top.nodes[node]).unwrap().get_id();
        // Get children of node and its parent
        let (par_lc, par_rc) = (
            ts.top.nodes[parent].get_lchild(),
            ts.top.nodes[parent].get_rchild(),
        );
        let (node_lc, node_rc) = (
            ts.top.nodes[node].get_lchild(),
            ts.top.nodes[node].get_rchild(),
        );
        // This vector will store all the nodes whose depth needs updating (required for correct Newick String generation later)
        let mut all_subnodes: Vec<usize>;

        if node.eq(&par_lc.unwrap()) {
            // left child of parent, swap right children
            new_topology.nodes[node].set_rchild(par_rc);
            new_topology.nodes[par_rc.unwrap()].set_parent(Some(node));
            new_topology.nodes[parent].set_rchild(node_rc);
            new_topology.nodes[node_rc.unwrap()].set_parent(Some(parent));
            all_subnodes = new_topology
                .postorder(&new_topology.nodes[par_rc.unwrap()])
                .chain(new_topology.postorder(&new_topology.nodes[node_rc.unwrap()]))
                .map(|n| n.get_id())
                .collect();
        } else {
            // right child of parent, swap left children
            new_topology.nodes[node].set_lchild(par_lc);
            new_topology.nodes[par_lc.unwrap()].set_parent(Some(node));
            new_topology.nodes[parent].set_lchild(node_lc);
            new_topology.nodes[node_lc.unwrap()].set_parent(Some(parent));
            all_subnodes = new_topology
                .postorder(&new_topology.nodes[par_lc.unwrap()])
                .chain(new_topology.postorder(&new_topology.nodes[node_lc.unwrap()]))
                .map(|n| n.get_id())
                .collect();
        };

        // This guarantees correct ordering of depth updating
        all_subnodes.sort();
        all_subnodes.reverse();
        // println!("all_subnodes: {:?}", all_subnodes);
        // Update depths in substrees that have been moved
        for n in all_subnodes {
            let d = new_topology
                .get_parent(&new_topology.nodes[n])
                .unwrap()
                .get_depth()
                + 1;
            new_topology.nodes[n].set_depth(d);
        }

        new_topology.tree_vec =
            newick_to_vector(&new_topology.get_newick(), new_topology.count_leaves());

        (Some(new_topology), None, Some(vec![node, parent]))
    }
}

// pub struct Dspsa{

// }

// impl<R: RateMatrix> TreeMove<R> for Dspsa{
//     fn generate(&self, ts: &TreeState<R>) -> TreeState<R> {
//         let mut rng = rand::thread_rng();
//         let distr = rand::distributions::Bernoulli::new(0.5).unwrap();

//         // Generate random peturbation vector
//         let mut delta: Vec<f64> = distr.sample_iter(rng).map(|r| if r {0.5} else {-0.5}).take(ts.top.tree_vec.len()).collect();

//         let phi: Vec<f64> = ts.top.tree_vec.iter().enumerate().map(|(i, v)|{
//             let x = *v as f64;
//             let ind = i as f64;
//             if x < 0.0 {
//                 0.0
//             } else if x > 2.0 * ind - 1.0 {
//                 ind - 0.0001
//             } else {
//                 x
//             }
//         }).collect();

//         let new_v: Vec<f64> = phi.iter().zip(delta.iter()).map(|(phi, delta)| phi + 0.5 + delta).collect();

//         todo!()
//     }
// }
