impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (i, num) in nums.iter().enumerate() {
            if i % 10 == *num as usize {
                return i as i32
            }
        }

        -1 as i32
    }
}