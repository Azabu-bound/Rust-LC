impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for (i, word) in words.iter().enumerate() {
            if word.contains(x) {
                result.push(i as i32);
            }
        }

        result
    }
}