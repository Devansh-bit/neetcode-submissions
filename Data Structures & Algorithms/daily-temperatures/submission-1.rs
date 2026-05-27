impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0i32; temperatures.len()];
        let n = temperatures.len();
        for i in (0..n.saturating_sub(1)).rev() {
            let mut j = i + 1;
            while j < n && temperatures[j] <= temperatures[i] {
                if res[j] == 0 {
                    j = n;
                    break;
                }
                j = res[j] as usize + j;
            }
            if j < n {
                res[i] = (j - i) as i32;
            }
        }
        res
    }
}
