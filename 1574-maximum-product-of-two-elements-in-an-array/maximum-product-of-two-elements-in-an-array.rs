impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut largest = i32::MIN;
        let mut second_largest = i32::MIN;


        for num in nums {
            if num > largest {
                second_largest = largest;
                largest = num;
            } else if num > second_largest {
                second_largest = num;
            }
        }

        (largest - 1) * (second_largest - 1)
    }
}