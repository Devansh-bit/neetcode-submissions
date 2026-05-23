fn two_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 2 { return vec![]; }
    let mut res = vec![];
    let mut right = nums.len() - 1;
    let mut left = 0;
    
    while left < right {
        let l = nums[left];
        let r = nums[right];
        
        if l + r < target {
            left += 1;
        } else if l + r > target {
            right -= 1;
        } else {
            res.push(vec![l, r]);
            left += 1;
            right -= 1;
            while left < nums.len() && nums[left] == nums[left - 1] { left += 1; }
            while left < right && nums[right] == nums[right + 1] { right -= 1; }
        }
    }
    res
}

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];
        let mut i = 0;
        
        while i + 2 < nums.len() {
            if nums[i] > 0 { break; } 
            let pairs = two_sum(&(nums[i + 1..]), -nums[i]);
            
            for mut pair in pairs.into_iter() {
                pair.push(nums[i]);
                res.push(pair);
            }
            
            i += 1;
            while i < nums.len() && nums[i] == nums[i - 1] { i += 1; }
        }
        res
    }
}