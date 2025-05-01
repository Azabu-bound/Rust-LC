impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let x_steps = (x-z).abs();
        let y_steps = (y-z).abs();
        
        if (x_steps < y_steps) {
            return 1;
        }

        if (x_steps > y_steps) {
            return 2;
        }

        0
    }
}