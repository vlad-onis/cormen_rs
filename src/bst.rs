use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

type NodeLink<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub left: Option<NodeLink<T>>,
    pub right: Option<NodeLink<T>>,
    pub parent: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct BinarySearchTree<T> {
    root: Option<NodeLink<T>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_with_parent(data: T, parent: &NodeLink<T>) -> Node<T> {
        Node {
            data,
            left: None,
            right: None,
            parent: Some(Rc::<RefCell<Node<T>>>::downgrade(parent)),
        }
    }
}

impl<T: PartialEq + PartialOrd + Debug> BinarySearchTree<T> {
    pub fn new() -> BinarySearchTree<T> {
        BinarySearchTree { root: None }
    }

    pub fn with_root(root: Rc<RefCell<Node<T>>>) -> BinarySearchTree<T> {
        BinarySearchTree { root: Some(root) }
    }

    pub fn insert(&mut self, value: T) -> bool {
        if let Some(root) = &self.root {
            Self::insert_impl(root, value)
        } else {
            let new_node = Rc::new(RefCell::new(Node::new(value)));
            self.root = Some(new_node);
            true
        }
    }

    fn insert_impl(current_node: &NodeLink<T>, data: T) -> bool {
        let mut node = current_node.borrow_mut();
        if node.data == data {
            false
        } else if data < node.data {
            match &node.left {
                Some(lnode) => {
                    return Self::insert_impl(lnode, data);
                }
                None => {
                    let new_node = Node::new_with_parent(data, current_node);
                    let new_node = Some(Rc::new(RefCell::new(new_node)));
                    node.left = new_node;
                    return true;
                }
            }
        } else {
            match &node.right {
                Some(rnode) => {
                    return Self::insert_impl(rnode, data);
                }
                None => {
                    let new_node = Node::new_with_parent(data, current_node);
                    let new_node = Some(Rc::new(RefCell::new(new_node)));
                    node.right = new_node;
                    return true;
                }
            }
        }
    }

    pub fn find(&self, data: T) -> bool {
        self.root
            .as_ref()
            .map_or(false, |root| Self::find_node(root, data).is_some())
    }

    fn find_node(start_node: &NodeLink<T>, data: T) -> Option<NodeLink<T>> {
        let current_node = &start_node.borrow();
        if current_node.data == data {
            Some(start_node.clone())
        } else if data < current_node.data {
            return current_node
                .left
                .as_ref()
                .and_then(|lnode| Self::find_node(lnode, data));
        } else {
            return current_node
                .right
                .as_ref()
                .and_then(|rnode| Self::find_node(rnode, data));
        }
    }
}

impl<T: PartialEq + PartialOrd + Debug> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn insert_parent() {
        let mut bst: BinarySearchTree<u32> = BinarySearchTree::new();

        bst.insert(10);
        bst.insert(20);
        bst.insert(5);
        bst.insert(7);
        bst.insert(3);
        println!("{:?}", bst);
    }

    #[test]
    pub fn find_node() {
        let mut bst: BinarySearchTree<u32> = BinarySearchTree::new();

        bst.insert(10);
        bst.insert(20);
        bst.insert(5);
        bst.insert(7);
        bst.insert(3);
        println!("{:?}", bst);

        assert!(bst.find(10));
        assert!(bst.find(20));
        assert!(bst.find(5));
        assert!(bst.find(7));
        assert!(bst.find(3));
        assert!(!bst.find(110));
        assert!(!bst.find(220));
    }
}
