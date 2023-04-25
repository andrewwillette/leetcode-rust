// https://leetcode.com/problems/smallest-even-multiple/description/
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n.rem_euclid(2) == 0 {
            n
        } else {
            n * 2
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test_smallest_even_multiple() {
        assert_eq!(10, super::Solution::smallest_even_multiple(5));
        assert_eq!(6, super::Solution::smallest_even_multiple(6));
    }
}
