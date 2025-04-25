impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result: Vec<i32> = vec![0; n * 2];

        for (i, num) in nums.iter().enumerate() {
            result[i] = *num;
            result[i + n] = *num;
        }

        result
    }
}