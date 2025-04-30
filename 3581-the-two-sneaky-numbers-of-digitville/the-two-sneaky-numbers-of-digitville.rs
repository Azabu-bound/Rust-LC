use std::collections::HashMap;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut hash = HashMap::new();
        let mut result = vec![];

        for num in nums {
            let count = hash.entry(num).or_insert(0);
            *count += 1;
            
            if *count == 2 {
                result.push(num);
            }
        }

        result
    }
}