use std::collections::HashMap;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut hash: HashMap<i32, i32> = HashMap::new();
        let mut result: Option<i32> = None;

        for num in nums {
            *hash.entry(num).or_insert(0) += 1;
        }

        hash.iter()
            .filter(|&(_, val)| *val == 2)
            .fold(0, |acc, (&key, _)| acc ^ key)
    }
}