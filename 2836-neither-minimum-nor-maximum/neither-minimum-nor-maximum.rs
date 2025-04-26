use std::cmp;

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 { return -1; }

        let min = cmp::min(nums[0], nums[1]);
        let max = cmp::max(nums[0], nums[1]);
        let res = nums[2];

        if res < max && res > min { return res; }
        if max < res && max > min { return max; }
        
        min
    }
}