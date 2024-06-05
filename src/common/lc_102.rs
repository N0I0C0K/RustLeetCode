struct Solution;

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

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut que: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut res = vec![];
        if root.is_none() {
            return res;
        }
        que.push_back(root.unwrap());
        while !que.is_empty() {
            let mut layer: Vec<i32> = vec![];
            for _ in 0..que.len() {
                let top = que.pop_front().unwrap();
                layer.push(top.borrow().val);
                if let Some(left) = top.borrow().left.as_ref() {
                    que.push_back(left.clone());
                }
                if let Some(right) = top.borrow().right.as_ref() {
                    que.push_back(right.clone());
                };
            }
            res.push(layer)
        }

        return res.into_iter().rev().collect::<Vec<Vec<i32>>>();
    }
}
