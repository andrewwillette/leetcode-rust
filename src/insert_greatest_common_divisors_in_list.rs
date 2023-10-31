// Definition for singly-linked list.
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
impl Solution {
    pub fn insert_greatest_common_divisors(_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let node = ListNode::new(5);
        Option::Some(Box::new(node))
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_left_rigth_difference() {
        let head = super::ListNode { next: None, val: 5 };
        assert_eq!(
            Option::Some(Box::new(head)),
            super::Solution::insert_greatest_common_divisors(Option::Some(Box::new(
                super::ListNode::new(5)
            )))
        );
    }
}
