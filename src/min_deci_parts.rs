impl Solution {
    /*
    # Leetcode problem # 1689
    leetcode.com/problems/partitioning-into-minimum-number-of-deci-binary-numbers
    */
    pub fn min_partitions(n: String) -> i32 {
        const RADIX: u32 = 10;
        let mut max: i32 = 0;
        for c in n.chars() {
            let i = c.to_digit(RADIX).unwrap();
            if i as i32 > max {
                max = i as i32;
            }
        }
        max
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_min_paritions() {
        assert_eq!(3, super::Solution::min_partitions(String::from("32")));
    }
}
