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
        let mut seen = vec![false; n];
        let mut ret = 0;
        for i in 0..n {
            if !seen[i] {
                seen[i] = true;
                ret += 1;
                let mut q = vec![i];
                while let Some(i) = q.pop() {
                    for &j in &similar[i] {
                        if !seen[j] {
                            seen[j] = true;
                            q.push(j);
                        }
                    }
                }
            }
        }
        ret
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_num_similar_groups() {
        assert_eq!(
            2,
            super::Solution::num_similar_groups(vec![
                "tars".to_string(),
                "rats".to_string(),
                "arts".to_string(),
                "star".to_string()
            ])
        );
    }
}
