impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; nums.len()];

        for i in 1..n {
            result[i] = result[i-1] * nums[i-1];
        }

        let mut suffix = 1;
        for i in (0..n).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }

        result
    }
}

