mod double_linked_list {
    use crate::main;

    pub struct List<T> {
        head: Link<T>,
        tail: Link<T>,
    }

    type Link<T> = Option<*mut Node<T>>;

    struct Node<T> {
        element: T,
        next: Link<T>,
        prev: Link<T>,
    }

    impl<T> Node<T> {
        fn new(element: T) -> *mut Self {
            //Using Box to allocate on the heap
            let node = Box::new(Node {
                element,
                prev: None,
                next: None,
            });
            Box::into_raw(node)
        }
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            return List {
                head: None,
                tail: None,
            };
        }

        pub fn push_front(&mut self, e: T) {
            let new_head = Node::new(e);
            unsafe {
                match self.head.take() {
                    Some(old_head) => {
                        (*old_head).prev = Some(new_head);
                        (*old_head).next = Some(old_head);
                    }
                    None => {
                        self.tail = Some(new_head);
                        self.head = Some(new_head);
                    }
                }
            }
        }

        pub fn push_back(&mut self, e: T) {
            let new_back = Node::new(e);
            unsafe {
                match self.tail.take() {
                    Some(old_back) => {
                        (*old_back).next = Some(new_back);
                        (*old_back).prev = Some(old_back);
                    }
                    None => {
                        self.tail = Some(new_back);
                        self.head = Some(new_back);
                    }
                }
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            unsafe {
                self.tail.take().map(|old_tail| {
                    match (*old_tail).prev.take() {
                        Some(new_tail) => {
                            (*new_tail).next.take();
                            // i.e. converting the reference to the popped element to None
                            self.tail = Some(new_tail);
                        }
                        None => {
                            // if the element being popped is the only element,
                            // also empty the head.
                            self.head.take();
                        }
                    }
                    // Reconstructing the `Box` allows the allocated heap to be properly freed.
                    let old_tail = Box::from_raw(old_tail);
                    old_tail.element
                })
            }
        }
    }
}
