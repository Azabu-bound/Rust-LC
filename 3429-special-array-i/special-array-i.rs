impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let n = nums.len();

        if n < 2 { return true; }

        let mut result = false;
        for i in 1..n {
            let prev_parity = nums[i - 1] % 2;
            let curr_parity = nums[i] % 2;

            if prev_parity != curr_parity {
                result = true;
            } else {
                return false;
            }
        }

        result
    }
}