impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_vol = 0;
        let mut l = 0;
        let mut r = heights.len() - 1;
        while l < r {
            let l_height = heights[l];
            let r_height = heights[r];
            let vol = (r - l) as i32 * l_height.min(r_height);
            max_vol = max_vol.max(vol);
            if l_height < r_height {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max_vol
    }
}
