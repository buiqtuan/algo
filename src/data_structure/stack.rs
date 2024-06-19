mod stack {
    use std::{cell::{Ref, RefCell}, fmt::Debug, rc::Rc};

    #[derive(Debug)]
    pub struct Node<T: Debug> {
        value: T,
        next: Option<Rc<RefCell<Node<T>>>>
    }

    #[derive(Debug)]
    pub struct Stack<T: Debug> {
        top: Option<Rc<RefCell<Node<T>>>>
    }

    struct IntoIter<T: Debug> (Option<Rc<RefCell<Node<T>>>>);

    impl<T: Debug> Iterator for IntoIter<T>  {
        type Item = T;
        
        fn next(&mut self) -> Option<Self::Item> {
            self.0.take().map(|current| {
                if let Some(next) = &current.borrow().next {
                    self.0 = Some(next.clone());
                }

                Rc::try_unwrap(current).ok().unwrap().into_inner().value
            })
        }
    }

    impl<T: Debug> Node<T> {
        fn new(value: T) -> Self {
            return Node {
                value: value,
                next: None
            }
        }
    }

    impl<T: Debug> Stack<T> {
        fn new() -> Self {
            return Stack {
                top: None
            }
        }

        fn push(&mut self, value: T) {
            let new_node = Node {
                value: value,
                next: self.top.clone()
            };
            self.top = Some(Rc::new(RefCell::new(new_node)));
        }

        fn pop(&mut self) -> Option<T> {
            self.top.take().map(|old_top| {
                if let Some(new_top) = &old_top.borrow().next {
                    self.top = Some(new_top.clone());
                }

                match Rc::try_unwrap(old_top) {
                    Ok(node) => {
                        node.into_inner().value
                    },
                    Err(rc) => {
                        panic!("Maybe there is another ref to this {rc:?}")
                    }
                }
            })
        }

        fn into_iter(self) -> IntoIter<T> {
            IntoIter(self.top)
        }
    }
}