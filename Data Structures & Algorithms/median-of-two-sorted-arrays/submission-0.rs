impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let (m, n) = (nums1.len(), nums2.len());
        let even = (m + n) % 2 == 0;
        let mut left: usize = 0;
        let mut right = m;

        while left <= right {
            let i = left + (right - left) / 2;
            let j = (m + n + 1) / 2 - i;

            let nums1_left  = i.checked_sub(1).map(|idx| nums1[idx]).unwrap_or(i32::MIN);
            let nums2_left  = j.checked_sub(1).map(|idx| nums2[idx]).unwrap_or(i32::MIN);
            let nums1_right = nums1.get(i).copied().unwrap_or(i32::MAX);
            let nums2_right = nums2.get(j).copied().unwrap_or(i32::MAX);

            if nums1_left <= nums2_right && nums2_left <= nums1_right {
                let max_left = nums1_left.max(nums2_left);
                return if even {
                    let min_right = nums1_right.min(nums2_right);
                    (max_left as f64 + min_right as f64) / 2.0
                } else {
                    max_left as f64
                };
            } else if nums1_left > nums2_right {
                right = i - 1; // safe: nums1_left != i32::MIN implies i > 0
            } else {
                left = i + 1;
            }
        }

        unreachable!("inputs must be sorted")
    }
}