impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut seen = HashMap::new();
        for word in strs.into_iter() {
            let mut arr = [0; 26];
            for c in word.bytes() {
                arr[(c - b'a') as usize] += 1;
            }
            seen.entry(arr).or_insert(vec![]).push(word);
        }
        seen.into_values().collect()
    }
}
