// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        for digit in digits {
            println!("{}", digit);
        }
        0
    }
}
pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_concatenation() {
        assert_eq!(52, super::Solution::minimum_sum(2932));
    }
}
