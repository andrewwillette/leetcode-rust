// https://leetcode.com/problems/find-the-original-array-of-prefix-xor
impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(pref.len());
        res.push(pref[0]);
        for i in 1..pref.len() {
            res.push(pref[i] ^ pref[i - 1])
        }
        res
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_array() {
        assert_eq!(
            vec![5, 7, 2, 3, 2],
            super::Solution::find_array(vec![5, 2, 0, 3, 1])
        );
    }
}
