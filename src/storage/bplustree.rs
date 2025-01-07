
struct Node <T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    is_leaf: bool,
    parent: Option<Box<Node<T>>>,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>,
}

struct BPlusTree {}
