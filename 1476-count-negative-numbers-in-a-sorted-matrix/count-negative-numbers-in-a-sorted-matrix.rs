impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        for row in grid {
            let i = row.partition_point(|&x| x >= 0);
            count += (row.len() - i) as i32;
        }

        count
    }
}