impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n == 0 { return -1; }
        let mut left = 0;
        let mut right = n-1;
        while left <= right {
            let mid_idx = (left + right)/2;
            let mid = nums[mid_idx];
            if mid == target {
                return mid_idx as i32;
            } else if mid > target {
                if mid_idx == 0 {break;}
                right = mid_idx.saturating_sub(1);
            } else {
                left = mid_idx + 1;
            }
        }
        return -1;
    }
}
