// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; nums.len()];
        let mut sorted: Vec<i32> = nums.clone();
        sorted.sort();
        for i in 0..nums.len() {
            for j in 0..sorted.len() {
                if nums[i] == sorted[j] {
                    result[i] = j as i32;
                    break;
                }
            }
        }
        result
    }
}

pub struct Solution;
