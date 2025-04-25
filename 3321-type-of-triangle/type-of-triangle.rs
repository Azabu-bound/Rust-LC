impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let x = nums[0];
        let y = nums[1];
        let z = nums[2];

        if x + y > z && y + z > x && x + z > y {
            match (x == y, y == z, x == z) {
                (true, true, true) => "equilateral".to_string(),
                (true, false, false) | (false, true, false) | (false, false, true) => "isosceles".to_string(),
                _ => "scalene".to_string(),
            }
        } else {
            "none".to_string()
        }

    }
}