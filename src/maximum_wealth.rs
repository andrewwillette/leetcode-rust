/**
 * https://leetcode.com/problems/richest-customer-wealth/
 */
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_wealth: i32 = 0;
        for customer in accounts.iter() {
            let wealth: i32 = customer.iter().sum();
            if wealth > max_wealth {
                max_wealth = wealth;
            }
        }
        max_wealth
    }
}

pub struct Solution;
