use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/deepest-leaves-sum/
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
struct Solution;
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}
