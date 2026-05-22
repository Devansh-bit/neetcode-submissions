impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut counts = [0; 26]; 
        let mut max_freq = 0;
        let mut left = 0;
        
        let s_bytes = s.as_bytes();

        for right in 0..s_bytes.len() {
            let idx = (s_bytes[right] - b'A') as usize;            
            counts[idx] += 1;            
            max_freq = max_freq.max(counts[idx]);

            if (right - left + 1) - max_freq > k as usize {
                counts[(s_bytes[left] - b'A') as usize] -= 1;
                left += 1;
            }
        }

        (s_bytes.len() - left) as i32
    }
}