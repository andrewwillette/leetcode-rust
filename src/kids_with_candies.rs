impl Solution {
    // kids_with_candies
    // https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let most: i32 = *candies.iter().max().unwrap();
        candies
            .iter()
            .map(|kid| kid + extra_candies >= most)
            .collect()
    }
}

pub struct Solution;
