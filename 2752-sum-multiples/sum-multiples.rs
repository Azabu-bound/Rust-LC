impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut nums: Vec<i32> = vec![];
        for num in 1..=n {
            if num % 7 == 0 {
                nums.push(num);
            } else if num % 5 == 0 {
                nums.push(num);
            } else if num % 3 == 0 {
                nums.push(num);
            }
        }
        nums.iter().sum()
    }
}