impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut x = 0;

        for operation in operations {
            x += if operation.contains('+') { 1 } else { - 1 };
        }

        x
    }
}