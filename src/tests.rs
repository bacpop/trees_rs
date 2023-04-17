#[cfg(test)]
mod tests {
    use crate::Tree;
    use crate::import;

    #[test]
    fn treemake() {
        let ts = String::from("4(2(3)(1))(6(5))");
        let tree = str2tree(ts, String::from("Tree1"));
        
        assert_eq!(tree.read_node(0).unwrap().parent, None);
        assert_eq!(tree.read_node(4).unwrap().parent, Some(0));
        assert_eq!(tree.read_parent(1).unwrap().children, (Some(1), Some(4)));
    }
}