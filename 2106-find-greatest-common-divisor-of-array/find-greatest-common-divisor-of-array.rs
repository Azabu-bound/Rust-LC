impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().unwrap(); 
        let min = nums.iter().min().unwrap();
        let n = max * min;
        let mut max_res = 1;

        for num in 2..=n {
            if max % num == 0 && min % num == 0 {
                max_res = num;
            }
        }

        max_res
    }
}