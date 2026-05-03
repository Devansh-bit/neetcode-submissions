impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {return false;}
        let mut counts: [i32; 26] = [0; 26];
        for (c1, c2) in s.bytes().zip(t.bytes()) {
            counts[(c1 - b'a') as usize] += 1;
            counts[(c2 - b'a') as usize] -= 1;
        }
        !counts.iter().any(|&n| n != 0)
    }
}