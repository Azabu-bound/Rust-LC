use itertools::Itertools;

impl Solution {
    pub fn transform_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = nums.clone();

        result.iter()
              .map(|&x| if x % 2 == 0 { 0 } else { 1 })
              .sorted()
              .collect::<Vec<i32>>()
    }
}