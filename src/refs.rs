use std::cell::RefCell;
use std::rc::Rc;

type GraphNodeRef<T> = Rc<RefCell<GraphNode<T>>>;

pub struct Graph<T> {
    pub root: Option<GraphNodeRef<T>>
}

pub struct GraphNode<T> {
    pub value: T,
    pub childs: Vec<GraphNodeRef<T>>,
}

impl<T> Graph<T> {
    pub fn new() -> Graph<T> {
        Graph::<T>{ root: None }
    }

    pub fn add_root_node(&mut self, value: T) -> GraphNodeRef<T> {
        let n = GraphNode::new(value);
        let rnode = Rc::new(RefCell::new(n));
        self.root = Some(rnode.clone());
        rnode
    }

    pub fn add_child(&mut self, node: GraphNodeRef<T>, value: T) -> GraphNodeRef<T> {
        let n = GraphNode::new(value);
        let rnode = Rc::new(RefCell::new(n));
        node.borrow_mut().childs.push(rnode.clone());
        rnode
    }

    pub fn add_child_node(&mut self, node: GraphNodeRef<T>, child: GraphNodeRef<T>) {
        node.borrow_mut().childs.push(child.clone())
    }

    pub fn child(&self, node: GraphNodeRef<T>, child: u32) -> GraphNodeRef<T> {
        node.borrow().childs[child as usize].clone()
    }
}

impl<T> GraphNode<T> {
    pub fn new(value: T) -> GraphNode<T> {
        GraphNode::<T>{ value: value, childs: vec![] }
    }
}
