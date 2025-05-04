impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_diff = 0;

        for i in 0..n {
            let next_i = (i + 1) % n;
            let diff = (nums[i] - nums[next_i]).abs();
            max_diff = std::cmp::max(max_diff, diff);
        }

        max_diff
    }
}