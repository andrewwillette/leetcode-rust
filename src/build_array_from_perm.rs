// https://leetcode.com/problems/build-array-from-permutation/
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).map(|i| nums[nums[i] as usize]).collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_left_rigth_difference() {
        assert_eq!(
            vec![0, 1, 2, 4, 5, 3],
            super::Solution::build_array(vec![0, 2, 1, 5, 3, 4])
        );
    }
}
