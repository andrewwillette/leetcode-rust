use std::cell::RefCell;
use std::rc::Rc;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

/// https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/description/
impl Solution {
    pub fn longest_zig_zag(root: OptNode) -> i32 {
        fn dfs(node: &OptNode, l_count: i32, r_count: i32) -> i32 {
            match node.as_ref() {
                Some(n) => {
                    let b = n.borrow();
                    dfs(&b.left, r_count + 1, 0).max(dfs(&b.right, 0, l_count + 1))
                }
                None => l_count.max(r_count),
            }
        }
        dfs(&root, 0, 0) - 1
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_zig_zag() {
        let mut tree = TreeNode::new(1);
        tree.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let _longest_zig_zag = Solution::longest_zig_zag(Some(Rc::new(RefCell::new(tree))));
    }
}

pub struct Solution;
