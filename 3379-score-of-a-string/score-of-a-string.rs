impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let n = s.len();
        let mut result = 0;

        for i in 1..n {
            let prev = s.as_bytes()[i - 1];
            let curr = s.as_bytes()[i];
            let diff = (prev as i32 - curr as i32).abs();
            
            result += diff;
        }

        result
    }
}