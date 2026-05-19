impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut seen: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for string in strs.into_iter() {
            let bytes = string.as_bytes();
            let mut counts = [0; 26];
            bytes.iter().for_each(|&b| {
                counts[(b - b'a') as usize] += 1;
            });
            seen.entry(counts).or_default().push(string);
        }
        seen.into_values().collect()
    }
}
