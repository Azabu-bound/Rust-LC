impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut sorted_nums: Vec<i32> = nums.clone();
        sorted_nums.sort_unstable();
        
        let w = sorted_nums[0];
        let x = sorted_nums[1];
        let y = sorted_nums[nums.len() - 2];
        let z = sorted_nums[nums.len() - 1];

        (y * z) - (w * x)
    }
}