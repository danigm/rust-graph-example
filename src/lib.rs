pub mod pointers;
pub mod hm;
//pub mod refs;

#[cfg(test)]
mod tests {

    #[test]
    fn raw_pointers() {
        use pointers::Graph;

        // graph:
        //
        // root
        // +- n1
        // +- n2
        //  +- n3
        //   +- n1
        let mut g: Graph<String> = Graph::new();
        g.add_root_node(String::from("root"));
        let root = g.root.unwrap();
        unsafe {
            let n1 = g.add_child(&mut *root, String::from("n1"));
            let n2 = g.add_child(&mut *root, String::from("n2"));
            let n3 = g.add_child(&mut *n2, String::from("n3"));
            (*n3).childs.push(n1);

            assert_eq!("root", (*root).value);
            assert_eq!("n1", (*(*root).childs[0]).value);
            assert_eq!("n2", (*(*root).childs[1]).value);
            assert_eq!("n3", (*(*n2).childs[0]).value);
            assert_eq!("n1", (*(*n3).childs[0]).value);

            assert_eq!(2, (*root).childs.len());
            assert_eq!(1, (*n2).childs.len());
            assert_eq!(1, (*n3).childs.len());
            assert_eq!(0, (*n1).childs.len());

            g.destroy();
        }
    }

    #[test]
    fn hash_map() {
        use hm::Graph;

        // graph:
        //
        // root
        // +- n1
        // +- n2
        //  +- n3
        //   +- n1
        let mut g: Graph<String> = Graph::new();
        g.add_root_node(String::from("root"));
        let root = 0;
        let n1 = g.add_child(root, String::from("n1"));
        let n2 = g.add_child(root, String::from("n2"));
        let n3 = g.add_child(n2, String::from("n3"));
        g.add_child_node(n3, n1);

        assert_eq!("root", g.nodes[&root].value);

        assert_eq!("n1", g.child(root, 0).value);
        assert_eq!("n2", g.child(root, 1).value);
        assert_eq!("n3", g.child(n2, 0).value);
        assert_eq!("n1", g.child(n3, 0).value);

        assert_eq!(2, g.nodes[&root].childs.len());
        assert_eq!(1, g.nodes[&n2].childs.len());
        assert_eq!(1, g.nodes[&n3].childs.len());
        assert_eq!(0, g.nodes[&n1].childs.len());
    }
}
