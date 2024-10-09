use crate::newick_to_vector;
use crate::tree::Tree;
use crate::rate_matrix::RateMatrix;
use rand::Rng;

// Rooted Nearest Neighbour Interchange
// Select random internal node - v
// Assign new parent as parent-of-parent
// Assign parent as having this node as parent

impl<T: RateMatrix> Tree<T> {

    pub fn nni(&mut self) -> () {
        // Get vector of all internal nodes that can be swapped
        let mut int_nodes: Vec<usize> = self.postorder_notips(self.get_root()).map(|node| node.index).collect();
        int_nodes.pop(); // Last element is root, which we can't swap with parent so pop it off
        let ind = rand::thread_rng().gen_range(0..int_nodes.len());
        let child_index = int_nodes[ind]; // was sample

        // Children and parent of selected node
        let child_node_children = self.nodes[child_index].children;
        let child_node_parent = self.nodes[child_index].parent.unwrap();

        // Children and parent of selected node's parent
        let (lp, rp) = self.nodes[child_node_parent].children;
        let parent_node_parent = self.nodes[child_node_parent].parent;

        // Assign new parent and children to selected node's parent
        self.nodes[child_node_parent].children = child_node_children;
        let (lc, rc) = child_node_children;

        // Update parents parent's children
        let parent_node_parent = self.nodes[child_node_parent].parent.unwrap();
        // if pp.is_none() {
            // Needed for when parent's parent is root
        // }

        let (lpc, rpc) = self.nodes[parent_node_parent].children;
        if lpc == Some(child_node_parent) {
            self.nodes[parent_node_parent].children = (Some(child_index), rpc);
        } else {
            self.nodes[parent_node_parent].children = (lpc, Some(child_index));
        }

        self.nodes[child_node_parent].parent = Some(child_index);
        // Update parent of moved children
        self.nodes[lc.unwrap()].parent = Some(child_node_parent);
        self.nodes[rc.unwrap()].parent = Some(child_node_parent);

        // Assign new parent and children to selected node
        self.nodes[child_index].parent = Some(parent_node_parent);

        if lp == Some(child_index) {
            self.nodes[child_index].children = (Some(child_node_parent), rp);
            self.nodes[rp.unwrap()].parent = Some(child_index);
        } else {
            self.nodes[child_index].children = (lp, Some(child_node_parent));
            self.nodes[lp.unwrap()].parent = Some(child_index);
        }

        // The new parent needs adding to changes for likelihood recalculation
        let d: usize = self.nodes[child_node_parent].depth;
        match self.changes.get(&d) {
            None => {
                self.changes.insert(d, vec![child_node_parent]);
            }
            Some(_) => {
                self.changes.get_mut(&d).unwrap().push(child_node_parent);
            }
        }

        // Update tree vector
        self.tree_vec = newick_to_vector(&self.newick(), self.count_leaves()); 
    }

}
