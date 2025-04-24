use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        let mut lucky: i32 = -1;
        let mut max_freq: i32 = 0;

        for &num in &arr {
            *hash.entry(num).or_insert(0) += 1;
        }

        for (num, freq) in &hash {
            if num == freq {
                if *freq > max_freq {
                    lucky = *num;
                    max_freq = *freq;
                }
            }
        }
        lucky
    }
}