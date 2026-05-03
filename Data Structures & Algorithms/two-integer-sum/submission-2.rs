use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = HashMap::with_capacity(nums.len());
        for (i, &val) in nums.iter().enumerate() {
            let to_check = target - val;
            if let Some(index) = seen.get(&to_check) {
                return vec![*index, i as i32];
            } else {
                seen.insert(val, i as i32);
            }
        }
        return vec![0, 0];
    }
}
