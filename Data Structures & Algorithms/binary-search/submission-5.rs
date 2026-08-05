impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // Cast pointers to i32 to avoid usize underflow panics
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2; 
            let mid_val = nums[mid as usize];

            if mid_val == target {
                return mid;
            } else if mid_val > target {
                right = mid - 1; 
            } else {
                left = mid + 1;
            }
        }
        
        -1
    }
}