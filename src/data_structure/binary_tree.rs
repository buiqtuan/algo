mod binary_tree {
    use std::{borrow::BorrowMut, cmp::Ordering, ops::Not};

    #[derive(Debug)]
    struct Node<T: Clone + Copy> {
        value: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    }

    #[derive(Debug)]
    pub struct BinaryTree<T: Clone + Copy> {
        root: Option<Box<Node<T>>>,
    }

    impl<T: Ord + std::fmt::Debug + Clone + Copy> BinaryTree<T> {
        pub fn new() -> Self {
            BinaryTree { root: None }
        }

        pub fn insert(&mut self, value: T) {
            Self::insert_node(&mut self.root, value);
        }

        fn remove(root: &mut Option<Box<Node<T>>>, value: T) {
            if let Some(node) = root {
                match &node.value.cmp(&value) {
                    Ordering::Equal => {
                        if node.left.is_none() && node.right.is_none() {
                            *root = None;
                        } else if node.left.is_none() {
                            *root = node.right.take();
                        } else if node.right.is_none() {
                            *root = node.left.take();
                        } else {
                            if let Some(min) = BinaryTree::min_value(node) {
                                node.value = min;
                                BinaryTree::remove(&mut node.right, min);
                            }
                        }
                    },
                    Ordering::Less => {
                        return BinaryTree::remove(&mut node.left, value);
                    },
                    Ordering::Greater => {
                        return BinaryTree::remove(&mut node.right, value);
                    }
                }
            }
        }

        fn min_value(node: &mut Box<Node<T>>) -> Option<T> {
            let mut current = node;
            while let Some(ref mut left) = current.left {
                current = left;
            }
            Some(current.value.clone())
        }

        fn insert_node(node: &mut Option<Box<Node<T>>>, value: T) {
            match node {
                None => {
                    *node = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                }
                Some(ref mut current) => match current.value.cmp(&value) {
                    std::cmp::Ordering::Equal => {
                        println!("Value {:?} already exists in the tree.", value)
                    }
                    std::cmp::Ordering::Greater => Self::insert_node(&mut current.left, value),
                    std::cmp::Ordering::Less => Self::insert_node(&mut current.right, value),
                },
            }
        }

        // Optional: Implement a function to display the tree, e.g., in-order traversal
    }
}
