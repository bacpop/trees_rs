#[cfg(test)]
mod tests {
    use crate::gen_list::Mutation;
    use crate::phylo2vec::phylo2vec_lin;
    use crate::phylo2vec::phylo2vec_quad;
    use crate::tree::Tree;
    // use crate::import::str2tree;
    // use crate::gen_list::Entry;
    // use crate::gen_list::MutationType;

    #[test]
    fn treemake_quad() {
        let mut tree = phylo2vec_quad(vec![0, 0, 0]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = phylo2vec_quad(vec![0, 0, 1]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = phylo2vec_quad(vec![0, 1, 0]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = phylo2vec_quad(vec![0, 1, 1]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = phylo2vec_quad(vec![0, 1, 2]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = phylo2vec_quad(vec![0, 1, 3]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);

        tree = phylo2vec_quad(vec![0, 0, 3]);

        assert_eq!(tree.get_node(0).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(1).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(2).unwrap().parent, Some(4));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(4).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().parent, Some(6));
        assert_eq!(tree.get_node(6).unwrap().parent, None);
    }

    #[test]
    fn phylo2vec_lin_same_as_phylo2vec_quad() {

        let vecs: Vec<Vec<usize>> = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 1, 2], vec![0, 1, 1]];
        let mut tree_q: Tree;
        let mut tree_l: Tree;

        for vec in vecs {
            let v = vec.clone();
            tree_q = phylo2vec_quad(v);
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

    #[test]
    fn update_tree_check() {
        let mut tree_q = phylo2vec_quad(vec![0, 1, 0]);
        let mut tree_l = phylo2vec_lin(vec![0, 0, 0], false);

        let vecs: Vec<Vec<usize>> = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 1, 2], vec![0, 1, 1]];

        for vec in vecs {
            let v = vec.clone();
            tree_q = phylo2vec_quad(v);
            tree_l.update_tree(Some(vec), false);

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
    // fn likelihood_multiplication_machinery() {
    //     let muts = Mutation(0.15, 0.5, 0.25, 0.1);

    //     let q: na::Matrix4<f64> = na::Matrix4::new(
    //         -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0,
    //     );

    //     let time = 0.75;

    //     let p = na::Matrix::exp(&(q * time));

    //     assert_eq!(p[(0, 0)], 0.6082994225745668);
    //     assert_eq!(p[(1, 2)], 0.5029001980127024);
    //     assert_eq!(p[(2, 1)], 0.5029001980127025);
    //     assert_eq!(p[(3, 3)], 0.6082994225745667);

    //     let ll = muts.child_likelihood(&p);

    //     assert_eq!(ll.1, 0.5187100816969821);
    //     assert_eq!(ll.3, 0.5292500041531686);

    //     // Check matrix multiplication works as expected
    //     assert_eq!(
    //         muts.0 * p[(0, 0)] + muts.1 * p[(0, 1)] + muts.2 * p[(0, 2)] + muts.3 * p[(0, 3)],
    //         ll.0
    //     );
    //     assert_eq!(
    //         muts.0 * p[(1, 0)] + muts.1 * p[(1, 1)] + muts.2 * p[(1, 2)] + muts.3 * p[(1, 3)],
    //         ll.1
    //     );
    //     assert_eq!(
    //         muts.0 * p[(2, 0)] + muts.1 * p[(2, 1)] + muts.2 * p[(2, 2)] + muts.3 * p[(2, 3)],
    //         ll.2
    //     );
    //     assert_eq!(
    //         muts.0 * p[(3, 0)] + muts.1 * p[(3, 1)] + muts.2 * p[(3, 2)] + muts.3 * p[(3, 3)],
    //         ll.3
    //     );

    //     // Check outcome of multiplying likelihoods from two child nodes
    //     let muts2 = Mutation(0.3, 0.1, 0.3, 0.1);
    //     let ll2 = muts2.child_likelihood(&p);

    //     let outcome = ll.prod(ll2);

    //     assert_eq!(outcome.0, ll.0 * ll2.0);
    //     assert_eq!(outcome.1, ll.1 * ll2.1);
    //     assert_eq!(outcome.2, ll.2 * ll2.2);
    //     assert_eq!(outcome.3, ll.3 * ll2.3);
    // }

    #[test]
    fn likelihood_internal_consistency() {
        let q: na::Matrix4<f64> = na::Matrix4::new(
            -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0,
        );

        let mut tr = phylo2vec_lin(vec![0, 0, 0], false);

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

    tr.update_tree(Some(vec![0, 0, 1]), false);
    tr.update_likelihood(&q);

    tr.update_tree(Some(vec![0, 0, 0]), false);
    tr.update_likelihood(&q);

    let new_likelihood = tr.get_tree_likelihood();

    assert_eq!(old_likelihood, new_likelihood);
    }

    // #[test]
    // fn likelihood_value_correct () {
    //     let q: na::Matrix4<f64> = na::Matrix4::new(
    //         -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0, 1.0, 1.0, 1.0, 1.0, -3.0,
    //     );
        
    //     let mut tr = phylo2vec_quad(vec![0; 1]);

    //     let genetic_data = vec![vec![Mutation(f64::ln(0.0), 0.0, f64::ln(0.0), f64::ln(0.0))], 
    //     vec![Mutation(f64::ln(0.0), f64::ln(0.0), f64::ln(0.0), 0.0)], 
    //     vec![]]; // This is the likelihood at the only internal (root) node, it can't be empty but will be overwritten

    //     tr.mutation_lists = genetic_data;

    //     tr.update_likelihood_postorder(&q);

    //     let likelihood = tr.get_tree_likelihood();

    //     assert_eq!(-2.773571667625644, likelihood);
    // }

    #[test]
    fn newick_test () {
        let mut tr = phylo2vec_quad(vec![0, 0, 0]);
        
        // Newick string for this tree is (1,(2,(3,0)4)5)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!(tr.get_node(4).unwrap().children, (Some(0), Some(3)));
        assert_eq!(tr.get_node(5).unwrap().children, (Some(4), Some(2)));
        assert_eq!(tr.get_node(6).unwrap().children, (Some(5), Some(1)));

        let mut tr = phylo2vec_quad(vec![0, 0, 1]);
        
        // Newick string for this tree is ((3,1)4,(2,0)5)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!(tr.get_node(4).unwrap().children, (Some(1), Some(3)));
        assert_eq!(tr.get_node(5).unwrap().children, (Some(0), Some(2)));
        assert_eq!(tr.get_node(6).unwrap().children, (Some(5), Some(4)));

        let mut tr = phylo2vec_quad(vec![0, 1, 1]);
        
        // Newick string for this tree is ((2,(3,1)4)5,0)6;
        // This should be the tree topology according to the ape package in R
        assert_eq!(tr.get_node(4).unwrap().children, (Some(1), Some(3)));
        assert_eq!(tr.get_node(5).unwrap().children, (Some(4), Some(2)));
        assert_eq!(tr.get_node(6).unwrap().children, (Some(0), Some(5)));

        let mut tr = phylo2vec_quad(vec![0, 1, 1, 3]);
        
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
}
