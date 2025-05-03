impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut even_idx = 0;
        let mut odd_idx = 0;
        let mut current = 0;

        while current < n {
            if current % 2 == 0 {
                while nums[even_idx] % 2 != 0 {
                    even_idx += 1;
                }
                nums.swap(current, even_idx);
                even_idx += 1;
            } else {
                while nums[odd_idx] % 2 == 0 {
                    odd_idx += 1;
                }
                nums.swap(current, odd_idx);
                odd_idx += 1;
            }
            current += 1;
        }

        nums
    }
}