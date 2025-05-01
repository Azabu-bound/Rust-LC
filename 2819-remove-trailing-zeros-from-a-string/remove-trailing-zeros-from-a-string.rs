impl Solution {
    pub fn remove_trailing_zeros(mut num: String) -> String {
        while num.ends_with('0') {
            num.pop();
        }

        num
    }
}