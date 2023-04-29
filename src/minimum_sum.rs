// https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        digits.sort_unstable();
        if let &[a, b, c, d] = &digits[..] {
            // use the smallest integers in 10 place
            (a * 10 + c + b * 10 + d) as i32
        } else {
            unreachable!()
        }
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
