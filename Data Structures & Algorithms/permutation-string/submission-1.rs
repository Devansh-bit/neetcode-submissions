impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();
        
        if n1 > n2 { 
            return false; 
        }
        
        // Removed 'mut' - fold returns the final initialized array
        let s1_counts = s1.bytes().fold([0; 26], |mut acc, ch| {
            acc[(ch - b'a') as usize] += 1;
            acc
        });

        let s2_bytes = s2.as_bytes();
        let mut s2_counts = [0; 26];

        // 1. Populate the initial window of size n1
        for i in 0..n1 {
            s2_counts[(s2_bytes[i] - b'a') as usize] += 1;
        }

        if s1_counts == s2_counts {
            return true;
        }

        // 2. Slide the window one byte at a time
        for i in n1..n2 {
            s2_counts[(s2_bytes[i] - b'a') as usize] += 1;
            s2_counts[(s2_bytes[i - n1] - b'a') as usize] -= 1;
            
            if s1_counts == s2_counts {
                return true;
            }
        }
        
        // Idiomatic expression return
        false
    }
}