impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut partial = vec![1; n];
        for i in 1..n {
            partial[i] = partial[i-1] * nums[i-1];
        }
        let mut suffix = 1;
        for i in (0..n).rev() {
            partial[i] = partial[i] * suffix;
            suffix = suffix * nums[i];
        }
        partial
    }
}
