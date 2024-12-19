#[cfg(test)]
use crate::always_accept;
use crate::apply_move;
use crate::create_dummy_gendata;
use crate::from_vec;
use crate::newick_to_vector;
use crate::random_vector;
use crate::rate_matrix::Gtr;
use crate::rate_matrix::RateMatrix;
use crate::ExactMove;
use crate::Topology;
use crate::TreeState;

#[test]
fn check_topology_build_manual() {
    // I check that new topologies have the correct parent by comparing to known parent values
    let mut top = from_vec(&[0, 0, 0, 0]);

    assert_eq!(top.nodes[0].get_parent(), Some(4));
    assert_eq!(top.nodes[0].get_parent(), Some(4));
    assert_eq!(top.nodes[1].get_parent(), Some(6));
    assert_eq!(top.nodes[2].get_parent(), Some(5));
    assert_eq!(top.nodes[3].get_parent(), Some(4));
    assert_eq!(top.nodes[4].get_parent(), Some(5));
    assert_eq!(top.nodes[5].get_parent(), Some(6));
    assert_eq!(top.nodes[6].get_parent(), None);

    top = from_vec(&[0, 0, 0, 1]);

    assert_eq!(top.nodes[0].get_parent(), Some(5));
    assert_eq!(top.nodes[1].get_parent(), Some(4));
    assert_eq!(top.nodes[2].get_parent(), Some(5));
    assert_eq!(top.nodes[3].get_parent(), Some(4));
    assert_eq!(top.nodes[4].get_parent(), Some(6));
    assert_eq!(top.nodes[5].get_parent(), Some(6));
    assert_eq!(top.nodes[6].get_parent(), None);

    top = from_vec(&[0, 0, 1, 0]);

    assert_eq!(top.nodes[0].get_parent(), Some(4));
    assert_eq!(top.nodes[1].get_parent(), Some(5));
    assert_eq!(top.nodes[2].get_parent(), Some(5));
    assert_eq!(top.nodes[3].get_parent(), Some(4));
    assert_eq!(top.nodes[4].get_parent(), Some(6));
    assert_eq!(top.nodes[5].get_parent(), Some(6));
    assert_eq!(top.nodes[6].get_parent(), None);

    top = from_vec(&[0, 0, 1, 1]);

    assert_eq!(top.nodes[0].get_parent(), Some(6));
    assert_eq!(top.nodes[1].get_parent(), Some(4));
    assert_eq!(top.nodes[2].get_parent(), Some(5));
    assert_eq!(top.nodes[3].get_parent(), Some(4));
    assert_eq!(top.nodes[4].get_parent(), Some(5));
    assert_eq!(top.nodes[5].get_parent(), Some(6));
    assert_eq!(top.nodes[6].get_parent(), None);

    top = from_vec(&[0, 0, 1, 2]);

    assert_eq!(top.nodes[0].get_parent(), Some(6));
    assert_eq!(top.nodes[1].get_parent(), Some(5));
    assert_eq!(top.nodes[2].get_parent(), Some(4));
    assert_eq!(top.nodes[3].get_parent(), Some(4));
    assert_eq!(top.nodes[4].get_parent(), Some(5));
    assert_eq!(top.nodes[5].get_parent(), Some(6));
    assert_eq!(top.nodes[6].get_parent(), None);

    top = from_vec(&[0, 0, 1, 3]);

    assert_eq!(top.nodes[0].get_parent(), Some(6));
    assert_eq!(top.nodes[1].get_parent(), Some(4));
    assert_eq!(top.nodes[2].get_parent(), Some(4));
    assert_eq!(top.nodes[3].get_parent(), Some(5));
    assert_eq!(top.nodes[4].get_parent(), Some(5));
    assert_eq!(top.nodes[5].get_parent(), Some(6));
    assert_eq!(top.nodes[6].get_parent(), None);

    top = from_vec(&[0, 0, 0, 3]);

    assert_eq!(top.nodes[0].get_parent(), Some(4));
    assert_eq!(top.nodes[1].get_parent(), Some(6));
    assert_eq!(top.nodes[2].get_parent(), Some(4));
    assert_eq!(top.nodes[3].get_parent(), Some(5));
    assert_eq!(top.nodes[4].get_parent(), Some(5));
    assert_eq!(top.nodes[5].get_parent(), Some(6));
    assert_eq!(top.nodes[6].get_parent(), None);
}

#[test]
fn update_tree() {
    let p = Gtr::default();
    let t_1 = from_vec(&[0, 0, 1, 0]);

    let mut gen_data = create_dummy_gendata(2, &t_1, &p.get_matrix());
    let ll = t_1.likelihood(&gen_data);

    let mut ts = TreeState {
        top: t_1,
        mat: p,
        likelihood: ll,
    };

    let vecs = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 2],
        vec![0, 0, 1, 1],
    ];
    let n = ts.top.nodes.len();

    for vec in vecs {
        let t_2 = from_vec(&vec);
        let mv = ExactMove { target_vector: vec };
        ts = apply_move(ts, mv, always_accept, &mut gen_data);

        for i in 0..n {
            assert_eq!(ts.top.nodes[i].get_parent(), t_2.nodes[i].get_parent());
            assert_eq!(ts.top.nodes[i].get_id(), t_2.nodes[i].get_id());
            // assert_eq!(ts.get_node(i).unwrap().get_id(), t_2.nodes[i].get_id());
        }
    }
}

#[test]
fn likelihood_internal_consistency_check() {
    let p = Gtr::default();
    let t = from_vec(&[0, 0, 0, 0]);
    let mut gen_data = create_dummy_gendata(5, &t, &p.get_matrix());
    let ll = t.likelihood(&gen_data);
    let mut ts = TreeState {
        top: t,
        mat: p,
        likelihood: ll,
    };

    let old_likelihood = ts.likelihood;

    let mv = ExactMove {
        target_vector: vec![0, 0, 0, 1],
    };

    ts = apply_move(ts, mv, always_accept, &mut gen_data);

    let mv = ExactMove {
        target_vector: vec![0, 0, 0, 0],
    };

    ts = apply_move(ts, mv, always_accept, &mut gen_data);

    let new_likelihood = ts.top.likelihood(&gen_data);

    assert_eq!(old_likelihood, new_likelihood);
}

#[test]
fn manual_parent_check() {
    let top = from_vec(&[0, 0, 0, 0]);
    // Newick string for this tree is (1,(2,(3,0)4)5)6;
    // This should be the tree topology according to the ape package in R
    assert_eq!(
        (top.nodes[4].get_lchild(), top.nodes[4].get_rchild()),
        (Some(0), Some(3))
    );
    assert_eq!(
        (top.nodes[5].get_lchild(), top.nodes[5].get_rchild()),
        (Some(4), Some(2))
    );
    assert_eq!(
        (top.nodes[6].get_lchild(), top.nodes[6].get_rchild()),
        (Some(5), Some(1))
    );

    let top = from_vec(&[0, 0, 0, 1]);
    // Newick string for this tree is ((3,1)4,(2,0)5)6;
    // This should be the tree topology according to the ape package in R
    assert_eq!(
        (top.nodes[4].get_lchild(), top.nodes[4].get_rchild()),
        (Some(1), Some(3))
    );
    assert_eq!(
        (top.nodes[5].get_lchild(), top.nodes[5].get_rchild()),
        (Some(0), Some(2))
    );
    assert_eq!(
        (top.nodes[6].get_lchild(), top.nodes[6].get_rchild()),
        (Some(5), Some(4))
    );

    let top = from_vec(&[0, 0, 1, 1]);
    // Newick string for this tree is ((2,(3,1)4)5,0)6;
    // This should be the tree topology according to the ape package in R
    assert_eq!(
        (top.nodes[4].get_lchild(), top.nodes[4].get_rchild()),
        (Some(1), Some(3))
    );
    assert_eq!(
        (top.nodes[5].get_lchild(), top.nodes[5].get_rchild()),
        (Some(4), Some(2))
    );
    assert_eq!(
        (top.nodes[6].get_lchild(), top.nodes[6].get_rchild()),
        (Some(0), Some(5))
    );

    let top = from_vec(&[0, 0, 1, 1, 3]);
    // Newick string for this tree is ((2,((4,3)5,1)6)7,0)8;
    // This should be the tree topology according to the ape package in R
    assert_eq!(
        (top.nodes[5].get_lchild(), top.nodes[5].get_rchild()),
        (Some(3), Some(4))
    );
    assert_eq!(
        (top.nodes[6].get_lchild(), top.nodes[6].get_rchild()),
        (Some(1), Some(5))
    );
    assert_eq!(
        (top.nodes[7].get_lchild(), top.nodes[7].get_rchild()),
        (Some(6), Some(2))
    );
    assert_eq!(
        (top.nodes[8].get_lchild(), top.nodes[8].get_rchild()),
        (Some(0), Some(7))
    );

    // R code:
    // mt <- ape::read.tree(text = "newick_string_here")
    // plot(mt)
}

#[test]
fn newick_vector_conversion_check() {
    let v = random_vector(27);
    let top = from_vec(&v);
    let nw = top.get_newick();
    let n_leaves = top.count_leaves();
    let y = newick_to_vector(&nw, n_leaves);
    assert_eq!(v, y);
    let trstr = from_vec(&y).get_newick();
    assert_eq!(trstr, nw);
}
