mod queue {
    //Implement a queue in a fixed size array.
    #[derive(Clone)]
    struct Queue<T> {
        data: [T;10000],
        reserve: Box<Vec<T>>,
        len: usize,
        head: usize,
        tail: usize
    }

    impl<T: Copy + Clone + Default + PartialEq> Queue<T> {
        fn new(d: T) -> Self {
            Queue {
                data: [d;10000],
                reserve: Box::new(vec![]),
                len: 0,
                head: 0,
                tail: 0
            }
        }

        fn enqueue(&mut self, value: T) {
            self.len += 1;

            self.data[self.head] = value;

            if self.head == self.data.len() - 1 {
                self.head = (self.head + 1) % self.data.len();
            } else {
                self.head += 1;
            }

            if self.tail == self.head {

                if self.data[self.tail] != T::default() {
                    self.reserve.push(self.data[self.tail]);
                }

                self.data[self.tail] = T::default();

                if self.tail == self.data.len() - 1 {
                    self.tail = (self.tail + 1) % self.data.len();
                } else {
                    self.tail += 1;
                }
            }
        }

        fn dequeue(&mut self) -> T {
            if self.len > 0 {
                self.len -= 1;
            } else {
                return T::default();
            }

            let value = self.data[self.tail];

            self.data[self.tail] = T::default();

            if self.tail == self.data.len() - 1 {
                self.tail = (self.tail + 1) % self.data.len();
            } else {
                self.tail += 1;
            }

            return value;
        }

        fn peek_head(&self) -> &T {
            return &self.data[self.head];
        }

        fn peek_tail(&self) -> &T {
            return &self.data[self.tail];
        }

        fn len(&self) -> &usize {
            return &self.len;
        }

        fn poll_reserve(&mut self) -> T {
            return self.reserve.pop().unwrap();
        }
    }

}