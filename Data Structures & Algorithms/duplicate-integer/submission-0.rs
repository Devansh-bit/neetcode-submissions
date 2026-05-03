use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new();
        nums.into_iter().any(|n| !seen.insert(n))
    }
}
