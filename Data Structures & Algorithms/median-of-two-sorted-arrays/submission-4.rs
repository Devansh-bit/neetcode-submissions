impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        
        // Helper function that executes the logic above
        fn get_kth(a: &[i32], b: &[i32], mut k: usize) -> i32 {
            // Base Case 1 & 2: If one array is empty, return the k-th from the other
            if a.is_empty() { return b[k - 1]; }
            if b.is_empty() { return a[k - 1]; }
            
            // Base Case 3: We just need the 1st element
            if k == 1 { return a[0].min(b[0]); }
            
            // Calculate step size (k/2). Prevent out-of-bounds if array is shorter than k/2.
            let i = a.len().min(k / 2);
            let j = b.len().min(k / 2);
            
            // Compare the elements and discard the appropriate chunk
            if a[i - 1] < b[j - 1] {
                // Discard the first 'i' elements of array A
                get_kth(&a[i..], b, k - i)
            } else {
                // Discard the first 'j' elements of array B
                get_kth(a, &b[j..], k - j)
            }
        }

        if total_len % 2 == 1 {
            get_kth(&nums1, &nums2, total_len / 2 + 1) as f64
        } else {
            let left_mid = get_kth(&nums1, &nums2, total_len / 2);
            let right_mid = get_kth(&nums1, &nums2, total_len / 2 + 1);
            (left_mid as f64 + right_mid as f64) / 2.0
        }
    }
}