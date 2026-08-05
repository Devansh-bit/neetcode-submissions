impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let min_idx = nums.partition_point(|&x| x > *nums.last().unwrap());
        println!("{}", min_idx);
        let result = if target <= nums[nums.len()-1] {
            min_idx + nums[min_idx..].partition_point(|&x| x < target)
        } else {
            nums[..min_idx].partition_point(|&x| x < target)
        };
        
        if result < nums.len() && result >= 0 && nums[result] == target {
            return result as i32;
        } else {
            return -1;
        }
    }
}
