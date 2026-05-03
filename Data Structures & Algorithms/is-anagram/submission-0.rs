impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // Anagrams must have the exact same length
        if s.len() != t.len() {
            return false;
        }

        let mut counts = [0; 26];

        // Increment counts for 's' and decrement for 't'
        for (c1, c2) in s.bytes().zip(t.bytes()) {
            counts[(c1 - b'a') as usize] += 1;
            counts[(c2 - b'a') as usize] -= 1;
        }

        // If it's a valid anagram, all elements in the array will be back to 0
        counts.iter().all(|&count| count == 0)
    }
}