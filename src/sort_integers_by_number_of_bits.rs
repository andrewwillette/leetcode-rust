// https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        println!("hi");
        for i in arr.iter() {
            println!("{:?}", i.to_be_bytes());
            println!("{:?}", Self::count_set_bits(*i))
        }
        let zero_vec: Vec<i32> = Vec::with_capacity(1);
        zero_vec
    }

    fn count_set_bits(value: i32) -> u32 {
        let mut count = 0;
        let mut num = value;

        while num != 0 {
            if num & 1 == 1 {
                count += 1;
            }
            num >>= 1;
        }

        count
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
