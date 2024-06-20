mod binary_heap {
    use std::{
        borrow::{Borrow, BorrowMut},
        collections::HashMap,
        hash::Hash, ops::Index,
    };

    struct MinHeap<T: Ord + Default + Hash + Copy> {
        data: Vec<T>,
        map: HashMap<T, Vec<usize>>,
    }

    impl<T: Ord + Default + Hash + Copy> MinHeap<T> {
        fn new() -> Self {
            return MinHeap {
                data: Vec::new(),
                map: HashMap::new(),
            };
        }

        pub fn push(&mut self, value: T) {
            let index = self.data.len() - 1;
            self.data.push(value);
            self.heapify_up(index);
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.data.len() > 1 {
                let value = self.data.swap_remove(0);
                self.heapify_down(0);
                Some(value)
            } else {
                self.map.clear();
                self.data.pop()
            }
        }

        fn heapify_up(&mut self, mut idx: usize) {
            while idx > 0 {
                let parent_idx = (idx - 1) / 2;
                if self.data[parent_idx] > self.data[idx] {
                    self.remove_and_push(parent_idx, idx);
                    self.remove_and_push(idx, parent_idx);

                    self.data.swap(idx, parent_idx);
                    idx = parent_idx;
                } else {
                    break;
                }
            }
        }

        fn heapify_down(&mut self, mut idx: usize) {
            let len = self.data.len();
            loop {
                let mut smallest_child = idx;
                let right_child = (2 * idx) + 2;
                let left_child = (2 * idx) + 1;

                if left_child < len && self.data[smallest_child] > self.data[left_child] {
                    smallest_child = left_child;
                }

                if right_child < len && self.data[smallest_child] > self.data[right_child] {
                    smallest_child = right_child;
                }

                if smallest_child != idx {
                    self.remove_and_push(smallest_child, idx);
                    self.remove_and_push(idx, smallest_child);

                    self.data.swap(smallest_child, idx);
                    idx = smallest_child;
                } else {
                    break;
                }
            }
        }

        pub fn remove(&mut self, value: T) {
            if self.map.contains_key(&value) {
                let o = self.map.get_mut(&value);
                match o {
                    Some(v) => {
                        let index = v.pop().unwrap();
                        //swap to the top then pop
                        self.data.swap(0, index);
                        self.pop();
                    },
                    None => {}
                }
            }
        }

        fn remove_and_push(&mut self, remove_i: usize, push_i: usize) {
            let v = self.map
                    .entry(self.data[remove_i])
                    .or_insert_with(Vec::new);
                v.remove(remove_i);
                v.push(push_i);
        }
    }
}
