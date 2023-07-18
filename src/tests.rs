#[cfg(test)]
mod tests {
    use crate::phylo2vec::phylo2vec_lin;
    use crate::phylo2vec::phylo2vec_quad;
    // use crate::tree::Tree;
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
    fn treemake_lin() {
        let mut tree_q = phylo2vec_quad(vec![0, 0, 0]);
        let mut tree_l = phylo2vec_lin(vec![0, 0, 0], false);

        assert_eq!(
            tree_l.get_node(0).unwrap().parent,
            tree_q.get_node(0).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(1).unwrap().parent,
            tree_q.get_node(1).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(2).unwrap().parent,
            tree_q.get_node(2).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(3).unwrap().parent,
            tree_q.get_node(3).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(4).unwrap().parent,
            tree_q.get_node(4).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(5).unwrap().parent,
            tree_q.get_node(5).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(6).unwrap().parent,
            tree_q.get_node(6).unwrap().parent
        );

        tree_q = phylo2vec_quad(vec![0, 1, 0]);
        tree_l = phylo2vec_lin(vec![0, 1, 0], false);

        assert_eq!(
            tree_l.get_node(0).unwrap().parent,
            tree_q.get_node(0).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(1).unwrap().parent,
            tree_q.get_node(1).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(2).unwrap().parent,
            tree_q.get_node(2).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(3).unwrap().parent,
            tree_q.get_node(3).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(4).unwrap().parent,
            tree_q.get_node(4).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(5).unwrap().parent,
            tree_q.get_node(5).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(6).unwrap().parent,
            tree_q.get_node(6).unwrap().parent
        );

        tree_q = phylo2vec_quad(vec![0, 1, 2]);
        tree_l = phylo2vec_lin(vec![0, 1, 2], false);

        assert_eq!(
            tree_l.get_node(0).unwrap().parent,
            tree_q.get_node(0).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(1).unwrap().parent,
            tree_q.get_node(1).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(2).unwrap().parent,
            tree_q.get_node(2).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(3).unwrap().parent,
            tree_q.get_node(3).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(4).unwrap().parent,
            tree_q.get_node(4).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(5).unwrap().parent,
            tree_q.get_node(5).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(6).unwrap().parent,
            tree_q.get_node(6).unwrap().parent
        );

        tree_q = phylo2vec_quad(vec![0, 1, 1]);
        tree_l = phylo2vec_lin(vec![0, 1, 1], false);

        assert_eq!(
            tree_l.get_node(0).unwrap().parent,
            tree_q.get_node(0).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(1).unwrap().parent,
            tree_q.get_node(1).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(2).unwrap().parent,
            tree_q.get_node(2).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(3).unwrap().parent,
            tree_q.get_node(3).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(4).unwrap().parent,
            tree_q.get_node(4).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(5).unwrap().parent,
            tree_q.get_node(5).unwrap().parent
        );
        assert_eq!(
            tree_l.get_node(6).unwrap().parent,
            tree_q.get_node(6).unwrap().parent
        );
    }

    //     #[test]
    //     fn relocatetree() {
    //         let ts = String::from("1(2(5(6))(4))(3)");
    //         let mut tree = str2tree(ts, String::from("Tree1"));

    //         assert_eq!(tree.get_node(3).unwrap().parent, Some(2));
    //         assert_eq!(tree.get_node(2).unwrap().children, (Some(3), None));

    //         tree.relocate(3, 5);

    //         assert_eq!(tree.get_node(2).unwrap().children, (None, None));
    //         assert_eq!(tree.get_node(3).unwrap().parent, Some(5));
    //         assert_eq!(tree.get_node(5).unwrap().children, (Some(3), None));
    //     }

    //     #[test]
    //     fn iteratetree() {
    //         let ts = String::from("1(2(5(6))(4))(3)");
    //         let tree = str2tree(ts, String::from("Tree1"));

    //         assert_eq!(tree.iter(tree.get_node(3)).fold(0,|acc, _node| acc + 1), 4);
    //         assert_eq!(tree.iter(tree.get_root()).fold(0,|acc, _node| acc + 1), 1);

    //         assert_eq!(tree.preorder(tree.get_root()).fold(0,|acc, _node| acc + 1), 6);
    //     }

    //     #[test]
    //     fn gen_list_entry() {
    //         let el: Entry = Entry::new('A', 1, Some(10));

    //         assert_eq!(el.start(), 1);
    //         assert_eq!(el.end(), Some(10));

    //     }
}
