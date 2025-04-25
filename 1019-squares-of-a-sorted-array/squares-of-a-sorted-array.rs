impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; nums.len()];
        let mut left = 0;
        let mut right = nums.len() - 1;

        for i in (0..nums.len()).rev() {
            let left_val = nums[left].abs();
            let right_val = nums[right].abs();

            if left_val > right_val {
                result[i] = left_val.pow(2);
                left += 1;
            } else {
                result[i] = right_val.pow(2);
                right -= 1;
            }
        }

        result
    }
}