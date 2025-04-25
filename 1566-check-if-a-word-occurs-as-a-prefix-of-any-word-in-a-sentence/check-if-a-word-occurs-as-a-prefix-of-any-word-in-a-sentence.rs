impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let words: Vec<String> = sentence.split_whitespace().map(String::from).collect();

        for (i, word) in words.iter().enumerate() {
            if word.starts_with(&search_word) {
                return (i + 1) as i32;
            }
        }

        -1
    }
}