impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1 as i32; nums.len()];
        for (index, val) in nums.into_iter().enumerate() {
            for (res_idx, res_val) in result.iter_mut().enumerate() {
                if res_idx == index {
                    continue
                }
                *res_val *= val;
            }
        }
        result
    }
}
