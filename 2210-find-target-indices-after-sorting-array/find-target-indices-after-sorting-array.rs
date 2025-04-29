impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort();
        let mut result = vec![];
        
        for (i, &num) in sorted.iter().enumerate() {
            if num == target {
                result.push(i as i32);
            }
        }
        
        result
    }
}