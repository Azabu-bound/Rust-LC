impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut sorted_nums1 = nums1.clone();
        sorted_nums1.sort_unstable();

        let mut sorted_nums2 = nums2.clone();
        sorted_nums2.sort_unstable();

        let x = sorted_nums1[0];
        let y = sorted_nums2[0];

        y - x
    }
}