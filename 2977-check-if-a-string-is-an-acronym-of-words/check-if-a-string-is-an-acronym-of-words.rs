impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        let mut result = String::new();

        for word in words {
            if let Some(ch) = word.chars().nth(0) {
                result.push(ch);
            }
        }

        result == s
    }
}