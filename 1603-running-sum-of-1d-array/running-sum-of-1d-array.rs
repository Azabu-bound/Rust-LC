impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut running_sum: Vec<i32> = vec![];

        let mut sum = 0;
        for num in &nums {
            sum += num;
            running_sum.push(sum);
        }

        running_sum
    }
}