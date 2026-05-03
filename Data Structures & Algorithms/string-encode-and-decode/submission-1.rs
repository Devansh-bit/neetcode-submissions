impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        // Scoped import: only applies inside this function, 
        // preventing collisions with LeetCode's global imports.
        use std::fmt::Write;

        let capacity = strs.iter().map(|s| s.len() + 5).sum();
        let mut result = String::with_capacity(capacity);

        for word in strs {
            let _ = write!(result, "{}#{}", word.len(), word);
        }
        
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let bytes = s.as_bytes(); 
        let mut i = 0;

        while i < bytes.len() {
            let mut len = 0;
            
            while bytes[i] != b'#' {
                len = len * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            
            i += 1; 

            let word = s[i..i + len].to_string();
            result.push(word);
            
            i += len; 
        }

        result
    }
}