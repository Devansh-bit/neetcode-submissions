impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut partial = vec![1; nums.len()];
        for i in 1..nums.len() {
            partial[i] = partial[i-1] * nums[i-1];
        }
        let mut suffix = vec![1; nums.len()];
        for i in (0..(nums.len()-1)).rev() {
            suffix[i] = nums[i+1] * suffix[i+1];
        }

        partial.iter().zip(suffix.iter()).map(|(x, y)| x * y).collect()
    }
}
