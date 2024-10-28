#[cfg(test)]
mod tests {
    use crate::newick_to_vec::*;
    use crate::rate_matrix::GTR;
    use crate::topology::Topology;

    #[test]
    fn check_topology_build_manual() {
        // I check that new topologies have the correct parent by comparing to known parent values
        let mut top: Topology = Topology::from_vec(&vec![0, 0, 0, 0]);

        assert_eq!(top.nodes[0].get_parent(), Some(4));
        assert_eq!(top.nodes[0].get_parent(), Some(4));
        assert_eq!(top.nodes[1].get_parent(), Some(6));
        assert_eq!(top.nodes[2].get_parent(), Some(5));
        assert_eq!(top.nodes[3].get_parent(), Some(4));
        assert_eq!(top.nodes[4].get_parent(), Some(5));
        assert_eq!(top.nodes[5].get_parent(), Some(6));
        assert_eq!(top.nodes[6].get_parent(), None);

        top = Topology::from_vec(&vec![0, 0, 0, 1]);

        assert_eq!(top.nodes[0].get_parent(), Some(5));
        assert_eq!(top.nodes[1].get_parent(), Some(4));
        assert_eq!(top.nodes[2].get_parent(), Some(5));
        assert_eq!(top.nodes[3].get_parent(), Some(4));
        assert_eq!(top.nodes[4].get_parent(), Some(6));
        assert_eq!(top.nodes[5].get_parent(), Some(6));
        assert_eq!(top.nodes[6].get_parent(), None);

        top = Topology::from_vec(&vec![0, 0, 1, 0]);

        assert_eq!(top.nodes[0].get_parent(), Some(4));
        assert_eq!(top.nodes[1].get_parent(), Some(5));
        assert_eq!(top.nodes[2].get_parent(), Some(5));
        assert_eq!(top.nodes[3].get_parent(), Some(4));
        assert_eq!(top.nodes[4].get_parent(), Some(6));
        assert_eq!(top.nodes[5].get_parent(), Some(6));
        assert_eq!(top.nodes[6].get_parent(), None);

        top = Topology::from_vec(&vec![0, 0, 1, 1]);

        assert_eq!(top.nodes[0].get_parent(), Some(6));
        assert_eq!(top.nodes[1].get_parent(), Some(4));
        assert_eq!(top.nodes[2].get_parent(), Some(5));
        assert_eq!(top.nodes[3].get_parent(), Some(4));
        assert_eq!(top.nodes[4].get_parent(), Some(5));
        assert_eq!(top.nodes[5].get_parent(), Some(6));
        assert_eq!(top.nodes[6].get_parent(), None);

        top = Topology::from_vec(&vec![0, 0, 1, 2]);

        assert_eq!(top.nodes[0].get_parent(), Some(6));
        assert_eq!(top.nodes[1].get_parent(), Some(5));
        assert_eq!(top.nodes[2].get_parent(), Some(4));
        assert_eq!(top.nodes[3].get_parent(), Some(4));
        assert_eq!(top.nodes[4].get_parent(), Some(5));
        assert_eq!(top.nodes[5].get_parent(), Some(6));
        assert_eq!(top.nodes[6].get_parent(), None);

        top = Topology::from_vec(&vec![0, 0, 1, 3]);

        assert_eq!(top.nodes[0].get_parent(), Some(6));
        assert_eq!(top.nodes[1].get_parent(), Some(4));
        assert_eq!(top.nodes[2].get_parent(), Some(4));
        assert_eq!(top.nodes[3].get_parent(), Some(5));
        assert_eq!(top.nodes[4].get_parent(), Some(5));
        assert_eq!(top.nodes[5].get_parent(), Some(6));
        assert_eq!(top.nodes[6].get_parent(), None);

        top = Topology::from_vec(&vec![0, 0, 0, 3]);

        assert_eq!(top.nodes[0].get_parent(), Some(4));
        assert_eq!(top.nodes[1].get_parent(), Some(6));
        assert_eq!(top.nodes[2].get_parent(), Some(4));
        assert_eq!(top.nodes[3].get_parent(), Some(5));
        assert_eq!(top.nodes[4].get_parent(), Some(5));
        assert_eq!(top.nodes[5].get_parent(), Some(6));
        assert_eq!(top.nodes[6].get_parent(), None);
    }

    // #[test]
    // fn update_tree() {
    //     let mut tree_1 = vector_to_tree(&vec![0, 0, 1, 0], &GTR::default());

    //     let vecs: Vec<Vec<usize>> = vec![vec![0, 0, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 2], vec![0, 0, 1, 1]];

    //     for vec in vecs {
    //         let tree_2 = vector_to_tree(&vec, &GTR::default());
    //         tree_1.update(&vec);

    //         for i in 0..=tree_1.tree_vec.len() {
    //             assert_eq!(
    //                 tree_1.get_node(i].get_parent(),
    //                 tree_2.get_node(i].get_parent());
    //             assert_eq!(
    //                 tree_1.get_node(i).unwrap().index,
    //                 tree_2.get_node(i).unwrap().index
    //             );
    //         }
    //     }
        
    // }
    
    // #[test]
    // fn likelihood_internal_consistency_check() {
    //     // let q: na::Matrix4<f64> = na::Matrix4::new(
    //     //     -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0,
    //     // );

    //     let mut tr = vector_to_tree(&vec![0, 0, 0, 0], &GTR::default());

    //     let genetic_data = vec![
    //     vec![
    //         Mutation(1.0, 0.0, 0.0, 0.0),
    //         Mutation(1.0, 0.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(0.0, 1.0, 0.0, 0.0),
    //         Mutation(1.0, 0.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(0.0, 0.0, 1.0, 0.0),
    //         Mutation(1.0, 0.0, 0.0, 0.0),
    //     ],
    //     vec![
    //         Mutation(1.0, 0.0, 0.0, 0.0),
    //         Mutation(0.0, 0.0, 0.0, 1.0),
    //     ],
    //     vec![
    //         Mutation(0.0, 1.0, 0.0, 0.0),
    //         Mutation(0.0, 0.0, 0.0, 1.0),
    //     ],
    //     vec![
    //         Mutation(0.0, 0.0, 1.0, 0.0),
    //         Mutation(0.0, 0.0, 0.0, 1.0),
    //     ],
    //     vec![
    //         Mutation(0.0, 1.0, 0.0, 0.0),
    //         Mutation(1.0, 0.0, 0.0, 0.0),
    //     ],
    // ];

    // tr.mutation_lists = genetic_data;

    // tr.initialise_likelihood();
    
    // let old_likelihood = tr.get_tree_likelihood();

    // tr.update(&vec![0, 0, 0, 1]);
    // tr.update_likelihood();

    // tr.update(&vec![0, 0, 0, 0]);
    // tr.update_likelihood();

    // let new_likelihood = tr.get_tree_likelihood();

    // assert_eq!(old_likelihood, new_likelihood);
    // }

    #[test]
    fn manual_parent_check () {

        let top: Topology = Topology::from_vec(&vec![0, 0, 0, 0]);
        // Newick string for this tree is (1,(2,(3,0)4)5)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!((top.nodes[4].get_lchild(), top.nodes[4].get_rchild()), (Some(0), Some(3)));
        assert_eq!((top.nodes[5].get_lchild(), top.nodes[5].get_rchild()), (Some(4), Some(2)));
        assert_eq!((top.nodes[6].get_lchild(), top.nodes[6].get_rchild()), (Some(5), Some(1)));

        let top: Topology = Topology::from_vec(&vec![0, 0, 0, 1]);        
        // Newick string for this tree is ((3,1)4,(2,0)5)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!((top.nodes[4].get_lchild(), top.nodes[4].get_rchild()), (Some(1), Some(3)));
        assert_eq!((top.nodes[5].get_lchild(), top.nodes[5].get_rchild()), (Some(0), Some(2)));
        assert_eq!((top.nodes[6].get_lchild(), top.nodes[6].get_rchild()), (Some(5), Some(4)));

        let top: Topology = Topology::from_vec(&vec![0, 0, 1, 1]);        
        // Newick string for this tree is ((2,(3,1)4)5,0)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!((top.nodes[4].get_lchild(), top.nodes[4].get_rchild()), (Some(1), Some(3)));
        assert_eq!((top.nodes[5].get_lchild(), top.nodes[5].get_rchild()), (Some(4), Some(2)));
        assert_eq!((top.nodes[6].get_lchild(), top.nodes[6].get_rchild()), (Some(0), Some(5)));

        let top: Topology = Topology::from_vec(&vec![0, 0, 1, 1, 3]);
        // Newick string for this tree is ((2,((4,3)5,1)6)7,0)8;
        // This should be the tree topology according to the ape package in R
        assert_eq!((top.nodes[5].get_lchild(), top.nodes[5].get_rchild()), (Some(3), Some(4)));
        assert_eq!((top.nodes[6].get_lchild(), top.nodes[6].get_rchild()), (Some(1), Some(5)));
        assert_eq!((top.nodes[7].get_lchild(), top.nodes[7].get_rchild()), (Some(6), Some(2)));
        assert_eq!((top.nodes[8].get_lchild(), top.nodes[8].get_rchild()), (Some(0), Some(7)));

        // R code:
        // mt <- ape::read.tree(text = "newick_string_here")
        // plot(mt)
    }

    #[test]
    fn newick_vector_conversion_check () {
        let v = random_vector(27);
        let top: Topology = Topology::from_vec(&v);
        let nw = top.get_newick();
        let n_leaves = top.count_leaves();
        let y = newick_to_vector(&nw, n_leaves);
        assert_eq!(v, y);
        let trstr = Topology::from_vec(&y).get_newick();
        assert_eq!(trstr, nw);
    }
}
