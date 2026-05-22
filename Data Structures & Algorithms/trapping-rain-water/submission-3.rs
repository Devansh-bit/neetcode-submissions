impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
    let (mut lo, mut hi) = (0, height.len() - 1);
    let (mut left_max, mut right_max) = (0, 0);
    let mut total = 0;

    while lo < hi {
        if height[lo] < height[hi] {
            left_max = left_max.max(height[lo]);
            total += left_max - height[lo];
            lo += 1;
        } else {
            right_max = right_max.max(height[hi]);
            total += right_max - height[hi];
            hi -= 1;
        }
    }
    total
}
}
