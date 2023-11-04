impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        1
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_last_moment() {
        assert_eq!(
            4,
            super::Solution::get_last_moment(4, vec![4, 3], vec![0, 1])
        )
    }
}
