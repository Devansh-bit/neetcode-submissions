impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::new();
        let mut res = vec![0i32; temperatures.len()];
        for (i, &temp) in temperatures.iter().enumerate() {
            // while current temp is larger than top of stack, pop, fill res
            // else push
            while !stack.is_empty() && temp > temperatures[*stack.last().unwrap()] {
                if let Some(top_idx) = stack.pop() {
                    res[top_idx] = (i - top_idx) as i32;
                }
            }
            stack.push(i);
        }
        res
    }
}
