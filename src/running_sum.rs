// https://leetcode.com/problems/running-sum-of-1d-array/
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        let mut running_sum: i32 = 0;
        for i in 0..nums.len() {
            running_sum += nums[i];
            nums[i] = running_sum;
        }
        nums
    }
}

pub struct Solution;
