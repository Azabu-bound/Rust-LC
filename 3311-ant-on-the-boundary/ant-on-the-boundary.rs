impl Solution {
    pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
        let mut current_place: i32 = 0;

        let mut result: i32 = 0;
        for num in &nums {
            current_place += num;
            if current_place == 0 { result += 1; }
        }

        result
    }
}