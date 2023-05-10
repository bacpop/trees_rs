#[cfg(test)]
mod tests {
    use crate::tree::Tree;
    use crate::import::str2tree;
    use crate::gen_list::Entry;
    use crate::gen_list::MutationType;

    #[test]
    fn treemake() {
        let ts = String::from("4(2(3)(1))(6(5))");
        let tree = str2tree(ts, String::from("Tree1"));
        
        assert_eq!(tree.get_node(0).unwrap().parent, None);
        assert_eq!(tree.get_node(4).unwrap().parent, Some(0));
        assert_eq!(tree.get_parent(1).unwrap().children, (Some(1), Some(4)));
        assert_eq!(tree.get_root().unwrap().parent, None);

        
    }

    #[test]
    fn relocatetree() {
        let ts = String::from("1(2(5(6))(4))(3)");
        let mut tree = str2tree(ts, String::from("Tree1"));

        assert_eq!(tree.get_node(3).unwrap().parent, Some(2));
        assert_eq!(tree.get_node(2).unwrap().children, (Some(3), None));

        tree.relocate(3, 5);

        assert_eq!(tree.get_node(2).unwrap().children, (None, None));
        assert_eq!(tree.get_node(3).unwrap().parent, Some(5));
        assert_eq!(tree.get_node(5).unwrap().children, (Some(3), None));
    }

    #[test]
    fn iteratetree() {
        let ts = String::from("1(2(5(6))(4))(3)");
        let tree = str2tree(ts, String::from("Tree1"));


        assert_eq!(tree.iter(tree.get_node(3)).fold(0,|acc, _node| acc + 1), 4);
        assert_eq!(tree.iter(tree.get_root()).fold(0,|acc, _node| acc + 1), 1);

        assert_eq!(tree.preorder(tree.get_root()).fold(0,|acc, _node| acc + 1), 6);
    }

    #[test]
    fn gen_list_entry() {
        let el: Entry = Entry::new('A', 1, Some(10));

        assert_eq!(el.start(), 1);
        assert_eq!(el.end(), Some(10));

    }
}