mod linked_list {
    use std::{cell::RefCell};
    use std::rc::Rc;


    struct Node<T> {
        value: T,
        next: Option<Rc<RefCell<Node<T>>>>
    }

    impl<T> Node<T> {
        fn new(value: T) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node {
                value,
                next: None
            }))
        }
    }

    struct LinkedList<T> {
        head: Option<Rc<RefCell<Node<T>>>>
    }

    impl<T> LinkedList<T> {
        fn new() -> Self {
            return LinkedList { head: None }
        }

        fn size(&self) -> usize {
            let mut size = 0;
            let mut current = self.head.clone();
            while let Some(node) = current {
                size += 1;
                current = node.borrow().next.clone();
            }
            size
        }
    }

    
}