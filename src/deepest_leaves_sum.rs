use std::cell::RefCell;
use std::rc::Rc;

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

// https://leetcode.com/problems/deepest-leaves-sum/
impl Solution {
    // find the first depth i where node is none, calculate sum at i - 1
    #[allow(dead_code)]
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        for i in 0.. {
            if Solution::get_node_at_depth(&root, i).is_none() {
                return Solution::get_sum_at_depth(&root, i - 1);
            }
        }
        unreachable!()
    }

    fn get_sum_at_depth(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize) -> i32 {
        match root.as_ref() {
            Some(n) => {
                let b = n.borrow();
                if depth == 0 {
                    b.val
                } else {
                    Solution::get_sum_at_depth(&b.left, depth - 1)
                        + Solution::get_sum_at_depth(&b.right, depth - 1)
                }
            }
            None => 0,
        }
    }

    // return the node at the given depth
    fn get_node_at_depth(
        root: &Option<Rc<RefCell<TreeNode>>>,
        depth: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root.as_ref() {
            Some(n) => {
                let b = n.borrow();
                if depth == 0 {
                    Some(Rc::clone(n))
                } else {
                    let left = Solution::get_node_at_depth(&b.left, depth - 1);
                    let right = Solution::get_node_at_depth(&b.right, depth - 1);
                    if left.is_some() {
                        left
                    } else {
                        right
                    }
                }
            }
            None => None,
        }
    }
}

#[allow(dead_code)]
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deepest_leaves_sum() {
        let mut tree = TreeNode::new(1);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        assert_eq!(
            1,
            Solution::deepest_leaves_sum(Some(Rc::new(RefCell::new(tree))))
        );
    }
}
