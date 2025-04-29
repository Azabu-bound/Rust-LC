use std::collections::HashMap;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let n = nums.len() / 2;
        let mut hash = HashMap::new();

        for num in nums {
            *hash.entry(num).or_insert(0) += 1;
        }

        for (key, val) in hash.iter() {
            if *val == n {
                return *key as i32;
            }
        }
        
        0
    }
}