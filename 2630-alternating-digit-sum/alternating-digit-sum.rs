impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let string = n.to_string();
        let mut result = 0;
        let mut parity = 0;

        for c in string.chars() {
            if parity % 2 == 0 {
                let num = c.to_digit(10).unwrap() as i32;
                result += num;
            } else {
                let num = c.to_digit(10).unwrap() as i32;
                result -= num;
            }
            
            parity += 1;
        }

        result
    }
}