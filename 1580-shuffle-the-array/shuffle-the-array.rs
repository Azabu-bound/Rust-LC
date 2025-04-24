impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut shuffled: Vec<i32>  = vec![];
        let mut left: i32 = 0;
        let mut right: i32 = n;

        while left < n {
            shuffled.push(nums[left as usize]);
            shuffled.push(nums[right as usize]);

            left += 1;
            right += 1;
        }

        shuffled
    }
}