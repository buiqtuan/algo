
use std::cmp::Ordering;

struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value: value,
            left: Subtree::new(),
            right: Subtree::new()
        }
    }
}

pub struct BinaryTree<T: Ord> {
    root: Subtree<T>
}

struct Subtree<T: Ord>(Option<Box<Node<T>>>);

impl<T: Ord> Subtree<T> {
    fn new() -> Self {
        Subtree(None)
    }

    fn insert(&mut self, value: T) {
        match &mut self.0 {
            None => self.0 =  Some(Box::new(Node::new(value))),
            Some(n) => {
                match value.cmp(&n.value) {
                    Ordering::Less => n.left.insert(value),
                    Ordering::Equal => {},
                    Ordering::Greater => n.right.insert(value)
                }
            }
        }
    }

    fn has(&self, value: T) -> bool {
        match &self.0 {
            None => return false,
            Some(n) => {
                match value.cmp(&n.value) {
                    Ordering::Equal => true,
                    Ordering::Less => n.left.has(value),
                    Ordering::Greater => n.right.has(value) 
                }
            }
        }
    }

    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len()
        }
    }
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self {
            root: Subtree::new()
        }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }
    
    fn has(&self, value: T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }   
}