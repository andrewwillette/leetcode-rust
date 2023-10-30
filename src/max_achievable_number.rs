// https://leetcode.com/problems/find-the-maximum-achievable-number/
impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        (t << 1) + num
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_the_maximum_achievable_x() {
        assert_eq!(6, super::Solution::the_maximum_achievable_x(4, 1));
    }
}
