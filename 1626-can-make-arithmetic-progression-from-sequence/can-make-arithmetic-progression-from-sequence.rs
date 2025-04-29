impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let n = arr.len();
        let mut sorted = arr.clone();
        sorted.sort_unstable();

        let mut prev_diff = sorted[1] - sorted[0];
        for i in 1..n-1 {
            let curr_diff = sorted[i + 1] - sorted[i];
            println!("curr diff: {}", curr_diff);
            println!("prev diff: {}", prev_diff);
            if curr_diff != prev_diff {
                return false;
            }

            prev_diff = curr_diff;
            println!("prev diff: {}", prev_diff);
        }

        true
    }
}