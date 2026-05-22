impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();

        let mut prefix = vec![0i32; n];
        let mut suffix = vec![0i32; n];

        suffix[n-1] = height[n-1];
        for i in (0..n-1).rev() {
            suffix[i] = suffix[i+1].max(height[i]);
        }

        prefix[0] = height[0];
        let mut prev_max = height[0];
        let mut total = suffix[0].min(prev_max).saturating_sub(height[0]);
        for i in 1..n {
            prev_max = prev_max.max(height[i]);
            total += suffix[i].min(prev_max).saturating_sub(height[i]);
        }

        total
    }
}
