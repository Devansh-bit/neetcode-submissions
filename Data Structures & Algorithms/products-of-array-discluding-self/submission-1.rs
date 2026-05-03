impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = vec![1; nums.len()];
        let mut index = 1;
        while index < nums.len() {
           prefix[index] = prefix[index-1] * nums[index-1];
           index += 1;
        }

        let mut suffix = vec![1; nums.len()];
        let mut index = nums.len() - 1;
        while index > 0 {
            index -= 1;
            suffix[index] = suffix[index+1] * nums[index+1];
        }

        prefix.into_iter().enumerate().map(|(i, p)| suffix[i]*p).collect()
    }
}
