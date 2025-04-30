impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let single: i32 = nums.iter().filter(|&x| x / 10 == 0).sum();
        let double: i32 = nums.iter().filter(|&x| x / 10 > 0).sum();

        single != double
    }
}