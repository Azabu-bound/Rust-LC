impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let max = a.max(b);
        let mut result = 0;

        for x in 1..=max {
            if a % x == 0 && b % x == 0 {
                result += 1;
            }
        }

        result
    }
}