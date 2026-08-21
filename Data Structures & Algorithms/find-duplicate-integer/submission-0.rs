impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0usize;
        let mut fast = 0usize;

        // Phase 1: advance until the pointers meet inside the cycle
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }

        // Phase 2: walk from the start and from the meeting point at equal speed;
        // they collide at the cycle entrance, which is the duplicate
        let mut slow2 = 0usize;
        while slow != slow2 {
            slow = nums[slow] as usize;
            slow2 = nums[slow2] as usize;
        }

        slow as i32
    }
}