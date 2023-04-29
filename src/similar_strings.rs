// https://leetcode.com/problems/similar-string-groups/
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let mut similar: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 0..n {
            for j in i + 1..n {
                let mut count = 0;
                for (c1, c2) in strs[i].chars().zip(strs[j].chars()) {
                    if c1 != c2 {
                        count += 1;
                        if count > 2 {
                            break;
                        }
                    }
                }
                if count <= 2 {
                    similar[i].push(j);
                    similar[j].push(i);
                }
            }
        }
        0
    }
}

pub struct Solution;
