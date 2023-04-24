impl Solution {
    /*
    # Leetcode problem # 1929
    https://leetcode.com/problems/concatenation-of-array/
    */
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut to_return = vec![0; nums.len() * 2];
        for i in 0..nums.len() {
            to_return[i] = nums[i];
            to_return[i + nums.len()] = nums[i];
        }
        to_return
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_concatenation() {
        assert_eq!(
            vec![1, 2, 1, 1, 2, 1],
            super::Solution::get_concatenation(vec![1, 2, 1])
        );
    }
}
