impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut product = 1;
        let mut sum = 0;
        let digits: Vec<u32> = n
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect();
        for digit in digits {
            product *= digit;
            sum += digit;
        }
        (product - sum) as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_concatenation() {
        assert_eq!(21, super::Solution::subtract_product_and_sum(4421));
    }
}
