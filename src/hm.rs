use std::collections::HashMap;

pub struct Graph<T> {
    pub nodes: HashMap<u32, GraphNode<T>>,
    pub idx: u32,
}

pub struct GraphNode<T> {
    pub value: T,
    pub childs: Vec<u32>,
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph::<T>{ idx: 0, nodes: HashMap::new() }
    }

    pub fn add_root_node(&mut self, value: T) {
        let n = GraphNode::new(value);
        self.nodes.insert(self.idx, n);
        self.idx += 1;
    }

    pub fn add_child(&mut self, node: u32, value: T) -> u32 {
        let n = GraphNode::new(value);
        let idx = self.idx;

        self.nodes.insert(idx, n);
        self.idx += 1;

        self.nodes.get_mut(&node).unwrap().childs.push(idx);
        idx
    }

    pub fn add_child_node(&mut self, node: u32, child: u32) {
        self.nodes.get_mut(&node).unwrap().childs.push(child);
    }

    pub fn child(&self, node: u32, child: u32) -> &GraphNode<T> {
        let idx = self.nodes.get(&node).unwrap().childs[child as usize];
        self.nodes.get(&idx).unwrap()
    }
}

impl<T> GraphNode<T> {
    pub fn new(value: T) -> GraphNode<T> {
        GraphNode::<T>{ value: value, childs: vec![] }
    }
}

