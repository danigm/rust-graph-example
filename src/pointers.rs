pub struct Graph<T> {
    pub root: Option<*mut GraphNode<T>>,
}

pub struct GraphNode<T> {
    pub value: T,
    pub childs: Vec<*mut GraphNode<T>>,
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph::<T>{ root: None }
    }

    pub fn add_root_node(&mut self, value: T) {
        let n = Box::new(GraphNode::new(value));
        self.root = Some(Box::into_raw(n));
    }

    pub fn add_child(&mut self, node: &mut GraphNode<T>, value: T) -> *mut GraphNode<T> {

        let n = Box::new(GraphNode::new(value));
        let pointer = Box::into_raw(n);
        node.childs.push(pointer);
        pointer
    }

    pub unsafe fn destroy(&mut self) {
        let mut removed: Vec<*mut GraphNode<T>> = vec![];

        if let Some(root) = self.root {
            let root = Box::from_raw(root);
            for i in 0..root.childs.len() {
                let c = root.childs[i];
                if removed.contains(&c) {
                    continue;
                }

                removed.push(c);
                // converting to box to remove
                let _ = Box::from_raw(c);
            }
        }
    }
}

impl<T> GraphNode<T> {
    pub fn new(value: T) -> GraphNode<T> {
        GraphNode::<T>{ value: value, childs: vec![] }
    }
}
