impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut result = 0;
        
        if a >= b {
            for x in 1..=a {
                if a % x == 0 && b % x == 0 {
                    result += 1;
                }
            }
        } else {
            for x in 1..=b {
                if a % x == 0 && b % x == 0 {
                    result += 1;
                }
            }
        }

        result
    }
}