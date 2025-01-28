use std::collections::HashMap;

#[allow(dead_code)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    is_leaf: bool,
    parent: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}
#[allow(dead_code)]
struct BPlusTree<T> {
    nodes: HashMap<u128, Node<T>>,
}
#[allow(dead_code)]
pub enum NodeType {
    Leaf,
    Regular,
}

