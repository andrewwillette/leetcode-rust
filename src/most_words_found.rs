// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut max_words: i32 = 0;
        for sentence in sentences {
            let words: Vec<&str> = sentence.split(' ').collect();
            if words.len() > max_words as usize {
                max_words = words.len() as i32;
            }
        }
        max_words
    }
}

pub struct Solution;
