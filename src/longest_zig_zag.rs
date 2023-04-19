use std::cell::RefCell;
use std::rc::Rc;

/// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/description/
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return 0;
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, isLeft: bool, count: i32) -> i32 {
        return 0;
    }
}

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

pub struct Solution;
