impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        // Size 26 perfectly maps A-Z using direct ASCII math (byte - 65)
        let mut counts = [0; 26]; 
        let mut max_freq = 0;
        let mut left = 0;
        
        let s_bytes = s.as_bytes();

        for right in 0..s_bytes.len() {
            // Convert ASCII byte to 0-25 index ('A' is 65)
            let idx = (s_bytes[right] - b'A') as usize;
            
            // Increment the count for the current character
            counts[idx] += 1;
            
            // Update the historical maximum frequency of a single character
            max_freq = max_freq.max(counts[idx]);

            // Core Equation: If characters to replace > k, the window is invalid
            if (right - left + 1) - max_freq > k as usize {
                // Remove the left-most character from our counts
                counts[(s_bytes[left] - b'A') as usize] -= 1;
                // Shift the left side of the window forward
                left += 1;
            }
        }

        // The final size of the window is guaranteed to be the maximum valid length
        (s_bytes.len() - left) as i32
    }
}