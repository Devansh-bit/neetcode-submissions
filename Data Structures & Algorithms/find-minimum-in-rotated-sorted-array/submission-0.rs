impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        nums[nums.partition_point(|&x| x > *nums.last().unwrap())]
    }
}
