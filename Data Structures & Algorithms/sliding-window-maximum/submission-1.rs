use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // Pre-allocate to avoid reallocations
        let mut res = Vec::with_capacity(nums.len().saturating_sub(k as usize - 1));
        let mut deque: VecDeque<usize> = VecDeque::new();

        for (i, &num) in nums.iter().enumerate() {
            // 1. Remove the index at the front if it's no longer in the window
            if let Some(&first_idx) = deque.front() {
                if first_idx + (k as usize) <= i {
                    deque.pop_front();
                }
            }

            // 2. Pop smaller elements from the back (they are useless now)
            while let Some(&last_idx) = deque.back() {
                if nums[last_idx] <= num {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            // 3. Add the current index
            deque.push_back(i);

            // 4. If our window has reached size k, record the max
            if i >= (k as usize) - 1 {
                // The front of the deque is always the maximum
                res.push(nums[*deque.front().unwrap()]);
            }
        }

        res
    }
}