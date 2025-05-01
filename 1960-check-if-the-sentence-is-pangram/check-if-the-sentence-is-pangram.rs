use std::collections::HashSet;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let set: HashSet<char> = sentence.chars().collect();

        set.len() == 26
    }
}