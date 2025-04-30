impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let mut count = 0.0;
        let n = s.len() as f64;

        for c in s.chars() {
            if c == letter {
                count += 1.0;
            }
            println!("count: {}", count);
        }

        (count / n * 100.0) as i32
    }
}