impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let (mut left, mut right) = (0, m);
        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n + 1) / 2 - i;

            let a_left = i.checked_sub(1).map(|idx| nums1[idx]).unwrap_or(i32::MIN);
            let a_right = nums1.get(i).copied().unwrap_or(i32::MAX);
            let b_left = j.checked_sub(1).map(|idx| nums2[idx]).unwrap_or(i32::MIN);
            let b_right = nums2.get(j).copied().unwrap_or(i32::MAX);

            if a_left <= b_right && b_left <= a_right {
                let left_max = a_left.max(b_left);
                if (m + n) % 2 == 0 {
                    let right_min = a_right.min(b_right);
                    return (left_max as f64 + right_min as f64) / 2 as f64;
                } else {
                    return left_max as f64;
                }
            }

            if a_left > b_right {
                right = i-1;
            }
            if b_left > a_right {
                left = i+1;
            }

        }
        unreachable!();
    }
}
