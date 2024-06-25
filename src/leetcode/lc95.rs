mod lc95 {
    // Definition for a binary tree node.
    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: Option<Rc<RefCell<TreeNode>>>,
        pub right: Option<Rc<RefCell<TreeNode>>>,
    }

    impl TreeNode {
        #[inline]
        pub fn new(val: i32) -> Self {
            TreeNode {
                val,
                left: None,
                right: None,
            }
        }
    }
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n < 2 {
            return vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))];
        } else {
            return generate_subtrees(1, n);
        }
    }

    fn generate_subtrees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut trees = Vec::new();

        if start > end {
            trees.push(None);
            return trees;
        }

        for i in start..=end {
            let left_trees = generate_subtrees(start, i - 1);
            let right_trees = generate_subtrees(i + 1, end);

            for left in left_trees.iter() {
                for right in right_trees.iter() {
                    let mut current = TreeNode::new(i);
                    current.left = left.clone();
                    current.right = right.clone();
                    trees.push(Some(Rc::new(RefCell::new(current))));
                }
            }
        }

        return trees;
    }

    use std::collections::HashMap;

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::<i32, i32>::new();

        for i in 0..nums.len() {
            let key = target - nums[i];

            if map.contains_key(&key) {
                return vec![i as i32, *map.get(&key).unwrap()];
            } else {
                map.insert(nums[i], i as i32);
            }
        }

        return vec![-1; 2];
    }

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode {
        pub val: i32,
        pub next: Option<Box<ListNode>>,
    }

    impl ListNode {
        #[inline]
        fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }
    }

    pub fn delete_duplicate(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.as_mut();

        while let Some(node) = current.borrow_mut() {
            while let Some(next) = node.next.as_mut() {
                if node.val == next.val {
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
        }

        head
    }
}
