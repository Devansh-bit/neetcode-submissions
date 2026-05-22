impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();

        let mut prefix = vec![0i32; n];
        let mut suffix = vec![0i32; n];

        prefix[0] = height[0];
        for i in 1..n {
            prefix[i] = prefix[i-1].max(height[i]);
        }

        suffix[n-1] = height[n-1];
        for i in (0..n-1).rev() {
            suffix[i] = suffix[i+1].max(height[i]);
        }

        let mut total = 0;
        for i in 0..n-1 {
            let volume_trapped = suffix[i].min(prefix[i]).saturating_sub(height[i]);
            total += volume_trapped;
        }

        total
    }
}
