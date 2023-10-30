// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits
impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by_key(|&x| (x.count_ones(), x));
        arr
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort_by_bits() {
        assert_eq!(
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7],
            super::Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
    }
}
