use std::cell::RefCell;
use std::rc::Rc;

/// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/description/
/// Not solved yet
impl Solution {
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            return std::cmp::max(
                Self::dfs(root.borrow().left, true, 0),
                Self::dfs(root.borrow().right, false, 0),
            );
        } else {
            return 0;
        }
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, is_left: bool, count: i32) -> i32 {
        if let Some(root) = root {
            if is_left {
                return std::cmp::max(
                    Self::dfs(root.borrow().left, true, 0),
                    Self::dfs(root.borrow().right, false, count + 1),
                );
            } else {
                return std::cmp::max(
                    Self::dfs(root.borrow().left, true, count + 1),
                    Self::dfs(root.borrow().right, false, 0),
                );
            }
        }
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
