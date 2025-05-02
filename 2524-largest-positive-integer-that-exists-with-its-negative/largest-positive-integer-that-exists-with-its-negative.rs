use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.clone().into_iter().collect();
        let mut max = -1;

        for num in nums {
            if num > max && set.contains(&-num) {
                max = num;
            }
        }

        max
        
    }
}