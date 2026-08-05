impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut max_area = 0 as i32;
        for (i, &height) in heights.iter().enumerate() {
            while let Some(&top_idx) = stack.last() {
                if height >= heights[top_idx] {
                    break;
                }
                stack.pop();
                let current_height = heights[top_idx];
                let width = if let Some(&left_idx) = stack.last() {
                    i - left_idx - 1
                } else {
                    i
                };
                max_area = max_area.max(width as i32 * current_height);
            }
            stack.push(i);
        }

        // cleanup, right boundary is n
        while let Some(top) = stack.pop() {
            let right_idx = heights.len() as i32;
            let left_idx = if let Some(&left) = stack.last() {
                left as i32
            } else {
                -1
            };
            let width = right_idx - left_idx - 1;
            max_area = max_area.max(heights[top] * width as i32);
        }

        return max_area;
    }
}
