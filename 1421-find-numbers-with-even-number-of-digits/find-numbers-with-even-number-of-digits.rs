impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut count_even = 0;

        for num in &nums {
            let mut temp = *num;
            let mut digits = 0;
            while temp > 0 {
                temp /= 10;
                digits += 1;
            }

            if digits % 2 == 0 { count_even += 1; }
        }

        count_even as i32
    }
}