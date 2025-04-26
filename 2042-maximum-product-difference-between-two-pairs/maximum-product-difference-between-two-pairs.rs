impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut largest = i32::MIN;
        let mut second_largest = i32::MIN;
        let mut smallest = i32::MAX;
        let mut second_smallest = i32::MAX;

        for num in nums {
            if num > largest {
                second_largest = largest;
                largest = num;
            } else if num > second_largest {
                second_largest = num;
            }
            
            if num < smallest {
                second_smallest = smallest;
                smallest = num;
            } else if num < second_smallest {
                second_smallest = num;
            }
        }

        (largest * second_largest) - (smallest * second_smallest)
    }
}