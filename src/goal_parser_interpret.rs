// https://leetcode.com/problems/goal-parser-interpretation/
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result = String::new();
        let letters = command.chars().collect::<Vec<char>>();
        for i in 0..letters.len() {
            match letters[i] {
                'G' => result.push('G'),
                '(' => {
                    if letters[i + 1] == ')' {
                        result.push('o');
                    } else {
                        result.push_str("al");
                    }
                }
                _ => continue,
            }
        }
        result
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    #[test]
    fn test_interpret() {
        assert_eq!("Goal", super::Solution::interpret(String::from("G()(al)")));
    }
}
