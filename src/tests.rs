#[cfg(test)]
mod tests {
    use crate::gen_list::Mutation;
    use crate::build_tree::phylo2vec_lin;
    use crate::build_tree::vector_to_tree;
    use crate::tree::Tree;
    use crate::build_tree::newick_to_vector;
    use crate::random_tree;

    #[test]
    fn treemake_quad() {
        let mut tree = vector_to_tree(&vec![0, 0, 0, 0]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = vector_to_tree(&vec![0, 0, 0, 1]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = vector_to_tree(&vec![0, 0, 1, 0]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = vector_to_tree(&vec![0, 0, 1, 1]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = vector_to_tree(&vec![0, 0, 1, 2]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = vector_to_tree(&vec![0, 0, 1, 3]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = vector_to_tree(&vec![0, 0, 0, 3]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);
    }

    #[test]
    fn phylo2vec_lin_same_as_vector_to_tree() {

        let vecs: Vec<Vec<usize>> = vec![vec![0, 0, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 2], vec![0, 0, 1, 1]];
        let mut tree_q: Tree;
        let mut tree_l: Tree;

        for vec in vecs {
            let v = vec.clone();
            tree_q = vector_to_tree(&v);
            tree_l = phylo2vec_lin(vec, false);

            for i in 0..=6 {
                assert_eq!(
                    tree_l.get_node(i).unwrap().parent,
                    tree_q.get_node(i).unwrap().parent);
                assert_eq!(
                    tree_l.get_node(i).unwrap().index,
                    tree_q.get_node(i).unwrap().index
                );
            }
        }

    }

    // #[test]
    // fn update_tree_check() {
    //     let mut tree_q = vector_to_tree(vec![0, 1, 0]);
    //     let mut tree_l = phylo2vec_lin(vec![0, 0, 0], false);

    //     let vecs: Vec<Vec<usize>> = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 1, 2], vec![0, 1, 1]];

    //     for vec in vecs {
    //         let v = vec.clone();
    //         tree_q = vector_to_tree(v);
    //         tree_l.update_quad(vec);

    //         for i in 0..=6 {
    //             assert_eq!(
    //                 tree_l.get_node(i).unwrap().parent,
    //                 tree_q.get_node(i).unwrap().parent);
    //             assert_eq!(
    //                 tree_l.get_node(i).unwrap().index,
    //                 tree_q.get_node(i).unwrap().index
    //             );
    //         }
    //     }
        
    // }

    #[test]
    fn update_tree_quad_check() {
        let mut tree_q = vector_to_tree(&vec![0, 0, 1, 0]);
        let mut tree_l = phylo2vec_lin(vec![0, 0, 0, 0], false);

        let vecs: Vec<Vec<usize>> = vec![vec![0, 0, 0, 0], vec![0, 0, 1, 0], vec![0, 0, 1, 2], vec![0, 0, 1, 1]];

        for vec in vecs {
            let v = vec.clone();
            tree_q = vector_to_tree(&v);
            tree_l.update(&vec);

            for i in 0..=6 {
                assert_eq!(
                    tree_l.get_node(i).unwrap().parent,
                    tree_q.get_node(i).unwrap().parent);
                assert_eq!(
                    tree_l.get_node(i).unwrap().index,
                    tree_q.get_node(i).unwrap().index
                );
            }
        }
        
    }
    
    #[test]
    fn likelihood_internal_consistency() {
        let q: na::Matrix4<f64> = na::Matrix4::new(
            -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0,
        );

        let mut tr = phylo2vec_lin(vec![0, 0, 0, 0], false);

        let genetic_data = vec![
        vec![
            Mutation(1.0, 0.0, 0.0, 0.0),
            Mutation(1.0, 0.0, 0.0, 0.0),
        ],
        vec![
            Mutation(0.0, 1.0, 0.0, 0.0),
            Mutation(1.0, 0.0, 0.0, 0.0),
        ],
        vec![
            Mutation(0.0, 0.0, 1.0, 0.0),
            Mutation(1.0, 0.0, 0.0, 0.0),
        ],
        vec![
            Mutation(1.0, 0.0, 0.0, 0.0),
            Mutation(0.0, 0.0, 0.0, 1.0),
        ],
        vec![
            Mutation(0.0, 1.0, 0.0, 0.0),
            Mutation(0.0, 0.0, 0.0, 1.0),
        ],
        vec![
            Mutation(0.0, 0.0, 1.0, 0.0),
            Mutation(0.0, 0.0, 0.0, 1.0),
        ],
        vec![
            Mutation(0.0, 1.0, 0.0, 0.0),
            Mutation(1.0, 0.0, 0.0, 0.0),
        ],
    ];

    tr.mutation_lists = genetic_data;

    tr.update_likelihood_postorder(&q);
    
    let old_likelihood = tr.get_tree_likelihood();

    tr.update(&vec![0, 0, 0, 1]);
    tr.update_likelihood(&q);

    tr.update(&vec![0, 0, 0, 0]);
    tr.update_likelihood(&q);

    let new_likelihood = tr.get_tree_likelihood();

    assert_eq!(old_likelihood, new_likelihood);
    }

    #[test]
    fn newick_test () {
        let mut tr = vector_to_tree(&vec![0, 0, 0, 0]);
        
        // Newick string for this tree is (1,(2,(3,0)4)5)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!(tr.get_node(4).unwrap().children, (Some(0), Some(3)));
        assert_eq!(tr.get_node(5).unwrap().children, (Some(4), Some(2)));
        assert_eq!(tr.get_node(6).unwrap().children, (Some(5), Some(1)));

        let mut tr = vector_to_tree(&vec![0, 0, 0, 1]);
        
        // Newick string for this tree is ((3,1)4,(2,0)5)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!(tr.get_node(4).unwrap().children, (Some(1), Some(3)));
        assert_eq!(tr.get_node(5).unwrap().children, (Some(0), Some(2)));
        assert_eq!(tr.get_node(6).unwrap().children, (Some(5), Some(4)));

        let mut tr = vector_to_tree(&vec![0, 0, 1, 1]);
        
        // Newick string for this tree is ((2,(3,1)4)5,0)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!(tr.get_node(4).unwrap().children, (Some(1), Some(3)));
        assert_eq!(tr.get_node(5).unwrap().children, (Some(4), Some(2)));
        assert_eq!(tr.get_node(6).unwrap().children, (Some(0), Some(5)));

        let mut tr = vector_to_tree(&vec![0, 0, 1, 1, 3]);
        
        // Newick string for this tree is ((2,((4,3)5,1)6)7,0)8;
        // This should be the tree topology according to the ape package in R
        assert_eq!(tr.get_node(5).unwrap().children, (Some(3), Some(4)));
        assert_eq!(tr.get_node(6).unwrap().children, (Some(1), Some(5)));
        assert_eq!(tr.get_node(7).unwrap().children, (Some(6), Some(2)));
        assert_eq!(tr.get_node(8).unwrap().children, (Some(0), Some(7)));

        // R code:
        // mt <- ape::read.tree(text = "newick_string_here")
        // plot(mt)
    }

    #[test]
    fn newick_conversion () {
        let mut tr = vector_to_tree(&random_tree(27));
        let nw = tr.newick();
        let n_leaves = tr.count_leaves();
        let y = newick_to_vector(&nw, n_leaves);
        let trstr = vector_to_tree(&y).newick();
        assert_eq!(trstr, nw);
    }
}
