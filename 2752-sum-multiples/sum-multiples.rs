impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        let mut result = 0;
        
        for num in 1..=n {
            if num % 7 == 0 || num % 5 == 0 || num % 3 == 0 {
                result += num;
            }
        }
        result
    }
}