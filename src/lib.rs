pub mod pointers;
//pub mod hm;
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

            g.destroy();
        }
    }
}
