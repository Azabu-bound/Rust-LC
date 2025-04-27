impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
       let mut sum_div_3 = 0;
       let mut count_div_3 = 0;

       for num in nums {
        if num % 3 == 0 && num % 2 == 0 {
            sum_div_3 += num;
            count_div_3 += 1;
        }
       }

        println!("sum: {}", sum_div_3);
        println!("count: {}", count_div_3);

       if count_div_3 == 0 { 0 } else { sum_div_3 / count_div_3 }
    }
}