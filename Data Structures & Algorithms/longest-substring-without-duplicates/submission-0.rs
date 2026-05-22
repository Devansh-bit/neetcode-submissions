impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen = [None; 128]; 
        let mut max_len = 0;
        let mut left = 0;

        for (right, &byte) in s.as_bytes().iter().enumerate() {
            let char_idx = byte as usize;

            // If we've seen the character before, jump the left pointer.
            if let Some(prev_idx) = last_seen[char_idx] {
                // left.max ensures we only move forward, ignoring duplicates 
                // that are already outside our current window.
                left = left.max(prev_idx + 1);
            }

            // Record the current index of the character
            last_seen[char_idx] = Some(right);

            // Update the maximum window size
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}