impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let last = *nums.last().unwrap();
        nums[nums.partition_point(|&x| x > last)]
    }
}