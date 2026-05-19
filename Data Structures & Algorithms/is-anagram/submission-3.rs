impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut counts = [0; 26];
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        for i in 0..s_bytes.len() {
            counts[(s_bytes[i] - b'a') as usize] += 1;
            counts[(t_bytes[i] - b'a') as usize] -= 1;
        }

        counts == [0; 26]
    }
}